pub mod dsp;
pub mod params;

mod config;
mod editor;
mod util;

use crate::{
    config::{get_plugin_config, PluginConfig},
    dsp::convolve::convolver,
    editor::create_editor,
};

use dsp::build_graph;
use fundsp::hacker32::*;
use nih_plug::prelude::*;
use params::PluginParams;
use rtrb::{Consumer, RingBuffer};

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
    // fundsp stuff
    graph: Box<dyn AudioUnit>,
    input_buffer: StereoBuffer,
    output_buffer: StereoBuffer,
    // for updating IR
    slot: Slot,
    slot_rx: Option<Consumer<Vec<f32>>>,
}
impl Default for ConvolutionPlug {
    fn default() -> Self {
        let graph = Box::new(pass());
        let slot = Slot::new(Box::new(pass())).0;

        Self {
            params: Arc::new(PluginParams::default()),
            graph,
            slot,
            slot_rx: None,
            input_buffer: StereoBuffer::new(),
            output_buffer: StereoBuffer::new(),
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

        (self.graph, self.slot) = build_graph(&self.params, &config);
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
        self.slot_rx = Some(ir_buffer_rx);

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
        for (_offset, mut block) in buffer.iter_blocks(MAX_BUFFER_SIZE) {
            // write into input buffer
            for (sample_index, mut channel_samples) in block.iter_samples().enumerate() {
                for channel_index in 0..=1 {
                    // get our input sample
                    let input_sample = *channel_samples.get_mut(channel_index).unwrap();

                    self.input_buffer.buffer_mut().set_f32(
                        channel_index,
                        sample_index,
                        input_sample,
                    );
                }
            }
            // actually do block processing
            self.graph.process(
                block.samples(),
                &self.input_buffer.buffer_ref(),
                &mut self.output_buffer.buffer_mut(),
            );

            // write from output buffer
            for (index, mut channel_samples) in block.iter_samples().enumerate() {
                for n in 0..=1 {
                    *channel_samples.get_mut(n).unwrap() =
                        self.output_buffer.buffer_ref().at_f32(n, index);
                }
            }
        }

        // update IR in our DSP graph

        // note that updating the IR in plugin's persistent data happens on the GUI thread
        // (because that requires locking a mutex which isn't RT safe)
        if let Some(rx) = self.slot_rx.as_mut() {
            // note that these samples should already be processed  from the gui thread
            // (normalized, whatever)
            if let Ok(new_ir_samples) = rx.pop() {
                // TODO: could use stacki here LOL
                let new_convolver =
                    Box::new(convolver(&new_ir_samples) | convolver(&new_ir_samples));
                self.slot.set(Fade::Smooth, 1.0, new_convolver);
            }
        }

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
