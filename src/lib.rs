pub mod dsp;
pub mod params;

mod config;
mod editor;
mod util;

use crate::{
    config::{get_plugin_config, PluginConfig},
    dsp::PluginDsp,
    editor::create_editor,
};

use fundsp::hacker32::*;
use nih_plug::prelude::*;
use params::PluginParams;
use rtrb::RingBuffer;

use std::sync::Arc;

// this is kind of silly in retrospect
type StereoBuffer = BufferArray<U2>;

// TODO:
// features:
// - predelay
// - decay / speed (something..) - maybe check convology
// - reverse (for fun LOL)

// TODO: make logging consistent and improve it in general
struct ConvolutionPlug {
    config: PluginConfig,
    params: Arc<PluginParams>,
    dsp: PluginDsp,
}
impl Default for ConvolutionPlug {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            dsp: PluginDsp::default(),
            config: PluginConfig::default(),
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

    // DO EXPENSIVE STUFF HERE
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("Building DSP graph..");

        let config = get_plugin_config();

        self.dsp.build_graph(&self.params, &config);

        self.config = config;
        nih_log!("Initialized Convolution.");

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        // this ring buffer is used to communicate that a new IR needs to be loaded
        let (ir_buffer_tx, ir_buffer_rx) = RingBuffer::<Vec<f32>>::new(1);
        self.dsp.set_slot_rx(ir_buffer_rx);

        Some(Box::new(create_editor(
            &self.params,
            ir_buffer_tx,
            &self.config,
        )))
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
