// #![warn(clippy::pedantic)]
// #![allow(clippy::wildcard_imports)]

pub mod dsp;
pub mod editor;
pub mod params;
pub mod processing;

use crate::{
    dsp::build_graph,
    editor::{
        tasks::{handle_task, Task},
        PluginGui,
    },
};

use fundsp::hacker32::*;
use nih_plug::prelude::*;

use nih_plug_webview::WebViewState;
use params::PluginParams;
use std::sync::{Arc, Mutex};

// TODO: make logging consistent and improve it in general
// TODO: make sure to log to a file

// TODO: improve documentation for functions and modules across the board

pub struct ConvolutionPlug {
    params: Arc<PluginParams>,
    graph: BigBlockAdapter,
    sample_rate: f32,
    slot: Arc<Mutex<Slot>>,
    buffers: Vec<Vec<f32>>,
}

const DEFAULT_SAMPLE_RATE: f32 = 44_100.0;
impl Default for ConvolutionPlug {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            graph: BigBlockAdapter::new(Box::new(sink())),
            slot: Arc::new(Mutex::new(Slot::new(Box::new(sink())).0)),
            sample_rate: DEFAULT_SAMPLE_RATE,
            buffers: Vec::new(),
        }
    }
}

impl Plugin for ConvolutionPlug {
    const NAME: &'static str = "Convolution";
    const VENDOR: &'static str = "dvub";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "todo";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    type SysExMessage = ();

    // handle expensive tasks
    type BackgroundTask = Task;
    fn task_executor(&mut self) -> TaskExecutor<Self> {
        handle_task(self)
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("Building DSP graph..");

        let config = self.params.ir_config.lock().unwrap();
        self.sample_rate = buffer_config.sample_rate;
        self.buffers = vec![vec![0.0; buffer_config.max_buffer_size as usize]; 2];

        match build_graph(&self.params, self.sample_rate, &config) {
            Ok((graph, slot)) => {
                let mut slot_lock = self.slot.lock().unwrap();
                *slot_lock = slot;

                self.graph = BigBlockAdapter::new(graph);
                self.graph
                    .set_sample_rate(f64::from(buffer_config.sample_rate));
                self.graph.allocate();

                nih_log!("Initialized Convolution.");
                true
            }
            Err(_) => false,
        }
    }

    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        PluginGui::new(
            &Arc::new(WebViewState::new(600.0, 600.0)),
            &self.params,
            async_executor,
        )
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for (i, chan) in buffer.as_slice_immutable().iter().enumerate() {
            self.buffers[i][..buffer.samples()].copy_from_slice(chan);
        }

        self.graph
            .process_big(buffer.samples(), &self.buffers, buffer.as_slice());

        ProcessStatus::Normal
    }
}

impl ClapPlugin for ConvolutionPlug {
    const CLAP_ID: &'static str = "com.your-domain.convolution-plug";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("todo");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // TODO: change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for ConvolutionPlug {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // TODO: change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(ConvolutionPlug);
nih_export_vst3!(ConvolutionPlug);
