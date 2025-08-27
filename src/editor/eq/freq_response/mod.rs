mod config;

use std::sync::{atomic::Ordering, Arc};

use fundsp::hacker32::*;
use nih_plug::prelude::AtomicF32;

use crate::{
    editor::{component::RenderingComponent, eq::freq_response::config::FrequencyResponseConfig},
    params::PluginParams,
};

pub struct FrequencyResponse {
    sample_rate: Arc<AtomicF32>,
    current_sample_rate: f32,
    config: FrequencyResponseConfig,
    graph: Box<dyn AudioUnit>,
    /// frequencies to grab a response from and display.
    frequencies: Vec<f32>,
}

const NUM_FREQS: usize = 100;

impl FrequencyResponse {
    pub fn new(p: Arc<PluginParams>, sample_rate: Arc<AtomicF32>) -> Self {
        let inc = (NUM_FREQS as f32).recip();
        let frequencies = (0..=NUM_FREQS).map(|x| x as f32 * inc).collect();

        Self {
            // TODO!!!!
            graph: Box::new(pass()),
            config: FrequencyResponseConfig::new(sample_rate.clone()),
            sample_rate: sample_rate.clone(),

            frequencies,
            current_sample_rate: 0.0,
        }
    }
}

impl RenderingComponent for FrequencyResponse {
    type RenderType = Vec<(f32, f32)>;

    fn tick(&mut self) {
        let new_sample_rate = self.sample_rate.load(Ordering::Relaxed);

        // TODO: fix this hack
        // check for a new sample rate
        if new_sample_rate != self.current_sample_rate {
            self.current_sample_rate = new_sample_rate;
            // NOTE: we do the divide by 2 here
            self.config.frequency_range.1 = new_sample_rate / 2.0;
            self.graph.set_sample_rate(new_sample_rate as f64);
        }

        self.graph.filter_stereo(0.5, 0.5);
    }

    fn get_drawing_coordinates(&mut self) -> Self::RenderType {
        self.frequencies
            .iter()
            .map(|normalized_frequency| {
                // TODO: calculate this in the constructor, if possible
                // sample rate thing kinda sucks
                let freq = normalized_to_value(*normalized_frequency, self.config.frequency_range);
                let response_db = self
                    .graph
                    .response_db(0, freq as f64)
                    .expect("ERROR GETTING FREQ RESPONSE") as f32;

                (freq, response_db)
            })
            .collect()
    }
}

fn normalized_to_value(normalized: f32, range: (f32, f32)) -> f32 {
    let minl = range.0.log2();
    let range = range.1.log2() - minl;
    2.0f32.powf((normalized * range) + minl)
}
