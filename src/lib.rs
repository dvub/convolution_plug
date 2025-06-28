pub mod dsp;
pub mod params;

mod config;
mod editor;
mod util;

use crate::{config::PluginConfig, dsp::build_graph, editor::create_editor};

use fundsp::hacker32::*;
use nih_plug::prelude::*;
use np_fundsp_bridge::PluginDspProcessor;
use params::PluginParams;
use std::sync::{Arc, Mutex};

// TODO:
// features:
// - predelay
// - decay / speed (something..) - maybe check convology
// - reverse (for fun LOL)

// TODO: make logging consistent and improve it in general
struct ConvolutionPlug {
    config: PluginConfig,
    params: Arc<PluginParams>,
    dsp: PluginDspProcessor<U2>,
    // this is used for updating the convolver
    slot: Arc<Mutex<Slot>>,
}
impl Default for ConvolutionPlug {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
            dsp: PluginDspProcessor::default(),
            config: PluginConfig::default(),
            slot: Arc::new(Mutex::new(Slot::new(Box::new(sink())).0)),
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

        // it's good to use a Result to say that this plugin could fail due to some OS issue
        // but when we use this function elsewhere, we don't want to panic or get fucked up because of a failure
        let config = PluginConfig::get_config().unwrap_or_else(|e| {
            nih_log!("There was an issue with reading the plugin config; Falling back to default for now.\n Error: {e}");
            PluginConfig::default()
        });
        let (graph, slot) = build_graph(&self.params, &config);

        // it is very important that we update the existing slot rather than simply overwriting it
        // (e.g. DO NOT DO: self.slot = Arc::new(Mutex::new(slot)))

        // when we update the existing slot, that will also make sure to update what our GUI thread is pointing to
        let mut slot_lock = self.slot.lock().unwrap();
        *slot_lock = slot;

        // otherwise updating all of this is trivial
        self.config = config;
        self.dsp.graph = graph;

        nih_log!("Initialized Convolution.");

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        Some(Box::new(create_editor(
            &self.params,
            &self.config,
            self.slot.clone(),
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
