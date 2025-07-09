// #![deny(clippy::unwrap_used)]
pub mod dsp;
pub mod params;

mod config;
pub mod editor;
mod util;

use crate::{dsp::build_graph, editor::create_editor};

use fundsp::hacker32::*;
use nih_plug::prelude::*;
use np_fundsp_bridge::PluginDspProcessor;
use params::PluginParams;
use std::sync::{Arc, Mutex};

// TODO: make logging consistent and improve it in general
// TODO: make sure to log to a file

// TODO: improve documentation for functions and modules across the board

pub struct ConvolutionPlug {
    params: Arc<PluginParams>,
    dsp: PluginDspProcessor<U2>,
    sample_rate: f32,
    // this is used for updating the convolver
    slot: Arc<Mutex<Slot>>,
}

const DEFAULT_SAMPLE_RATE: f32 = 44_100.0;
impl Default for ConvolutionPlug {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            dsp: PluginDspProcessor::default(),

            slot: Arc::new(Mutex::new(Slot::new(Box::new(sink())).0)),
            sample_rate: DEFAULT_SAMPLE_RATE,
        }
    }
}

impl Plugin for ConvolutionPlug {
    const NAME: &'static str = "Convolution";
    const VENDOR: &'static str = "dvub";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "todo";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

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

        let config = self.params.config.lock().unwrap().clone();

        // TODO: initialize() should return false if anything fails, including locking the slot mutex
        let (mut graph, slot) = build_graph(&self.params, &config, buffer_config.sample_rate)
            .expect("There was an issue building the graph");

        graph.set_sample_rate(buffer_config.sample_rate as f64);

        let mut slot_lock = self.slot.lock().unwrap();
        *slot_lock = slot;

        self.dsp.graph = graph;
        self.sample_rate = buffer_config.sample_rate;

        nih_log!("Initialized Convolution.");

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        Some(Box::new(create_editor(self)))
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.dsp.process(buffer);
        ProcessStatus::Normal
    }
}

impl ClapPlugin for ConvolutionPlug {
    const CLAP_ID: &'static str = "com.your-domain.convolution-plug";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("todo");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for ConvolutionPlug {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(ConvolutionPlug);
nih_export_vst3!(ConvolutionPlug);
