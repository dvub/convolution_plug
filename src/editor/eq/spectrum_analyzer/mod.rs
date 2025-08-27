pub mod monitor;

use monitor::Monitor;

use crossbeam_channel::Receiver;
use fundsp::hacker32::*;
use nih_plug::prelude::AtomicF32;
use rayon::prelude::*;
use std::sync::{atomic::Ordering, Arc, Mutex};

use crate::editor::{component::RenderingComponent, eq::spectrum_analyzer::monitor::MonitorMode};

const WINDOW_LENGTH: usize = 4096;
const NUM_MONITORS: usize = (WINDOW_LENGTH / 2) + 1;

// roughly -78 db
const MIN_GAIN: f32 = 0.000126;
pub struct SpectrumAnalyzer {
    // NOTE: we could probably compute the FFT without fundsp
    // (but i like fundsp)
    graph: Box<dyn AudioUnit>,

    sample_rx: Receiver<f32>,

    spectrum: Arc<Mutex<Vec<f32>>>,
    spectrum_monitors: Vec<Monitor>,
    output_buf: Vec<f32>,
}

const DEFAULT_PEAK_DECAY: f32 = 0.25; // seconds

pub const DEFAULT_MONITOR_MODE: MonitorMode = MonitorMode::Rms(DEFAULT_PEAK_DECAY);

impl SpectrumAnalyzer {
    pub fn new(sample_rate: Arc<AtomicF32>, sample_rx: Receiver<f32>) -> Self {
        let spectrum_monitors = vec![Monitor::new(DEFAULT_MONITOR_MODE); NUM_MONITORS];
        let spectrum = Arc::new(Mutex::new(vec![0.0; NUM_MONITORS]));

        let mut graph = build_fft_graph(spectrum.clone());
        // TODO: is this needed?1
        graph.set_sample_rate(sample_rate.load(Ordering::Relaxed) as f64);

        Self {
            spectrum,
            spectrum_monitors,
            graph,
            sample_rx,
            output_buf: vec![0.0; NUM_MONITORS],
        }
    }

    fn get_bin_levels(&mut self) {
        let spectrum = &*self.spectrum.lock().unwrap();
        self.spectrum_monitors
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| {
                x.tick(spectrum[i]);
                self.output_buf[i] = x.level();
            });
    }
    /*
    pub fn set_monitor_mode(&mut self, meter: monitor::MonitorMode) {
        for mon in self.spectrum_monitors.iter_mut() {
            mon.set_mode(meter);
        }
    }

    pub fn set_monitor_fps(&mut self, frame_rate: f32) {
        for mon in self.spectrum_monitors.iter_mut() {
            mon.set_frame_rate(frame_rate);
        }
    }
    pub fn set_monitor_decay_speed(&mut self, speed: f32) {
        for mon in self.spectrum_monitors.iter_mut() {
            mon.set_decay_speed(speed);
        }
    }
    */
}

impl RenderingComponent for SpectrumAnalyzer {
    type RenderType = Vec<f32>;

    fn tick(&mut self) {
        for sample in self.sample_rx.try_iter() {
            self.graph.tick(&[sample], &mut [])
        }
    }
    // TODO: fix clone?
    fn get_drawing_coordinates(&mut self) -> Self::RenderType {
        self.get_bin_levels();
        if self.output_buf.par_iter().all(|x| *x < MIN_GAIN) {
            return Vec::new();
        }
        self.output_buf.clone()
    }
}

fn build_fft_graph(spectrum: Arc<Mutex<Vec<f32>>>) -> Box<dyn AudioUnit> {
    let normalization = WINDOW_LENGTH as f32 / 2.0;

    let fft_processor = resynth::<U1, U0, _>(WINDOW_LENGTH, move |fft| {
        let mut spectrum = spectrum.lock().unwrap();

        #[allow(clippy::needless_range_loop)]
        for i in 0..fft.bins() {
            let current_bin = fft.at(0, i);
            let value = current_bin.norm() / normalization;
            spectrum[i] = value;
        }
    });

    Box::new(fft_processor)
}
