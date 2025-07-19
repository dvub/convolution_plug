// #![warn(clippy::unwrap_used)]
// #![warn(clippy::expect_used)]
pub mod dsp;
pub mod params;

mod config;
pub mod editor;
mod util;

use crate::{
    config::IrConfig,
    dsp::{
        build_graph,
        ir::{init_convolvers, process_ir},
    },
    editor::{
        create_editor,
        event_loop::{FADE_TIME, FADE_TYPE},
        ipc::IrData,
    },
    util::decode_samples,
};

use fundsp::hacker32::*;
use nih_plug::prelude::*;

use params::PluginParams;
use std::sync::{Arc, Mutex};

// TODO: make logging consistent and improve it in general
// TODO: make sure to log to a file

// TODO: improve documentation for functions and modules across the board

pub struct ConvolutionPlug {
    params: Arc<PluginParams>,
    graph: BigBlockAdapter,
    sample_rate: f32,
    // this is used for updating the convolver
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

pub enum Task {
    UpdateIrConfig(IrConfig),
    UpdateIr(IrData),
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

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = Task;

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        let params = self.params.clone();
        let slot = self.slot.clone();
        let sample_rate = self.sample_rate;

        // TODO: refactor
        // TODO: fix an ungodly amount of unwrawps
        Box::new(move |task| match task {
            Task::UpdateIrConfig(new_ir_config) => {
                let (ir_samples, ir_sample_rate) = &*params.ir_samples.lock().unwrap();

                // if IR is already loaded
                if !ir_samples.is_empty() {
                    let processed_ir =
                        process_ir(ir_samples, *ir_sample_rate, sample_rate, &new_ir_config)
                            .unwrap();

                    let convolvers = init_convolvers(&processed_ir);

                    slot.lock().unwrap().set(FADE_TYPE, FADE_TIME, convolvers);
                }

                // regardless of if IR is loaded, make sure the new config is persistent
                *params.ir_config.lock().unwrap() = new_ir_config;
            }
            Task::UpdateIr(ir_data) => {
                let config = params.ir_config.lock().unwrap();

                let (ir_samples, ir_sample_rate) = decode_samples(&ir_data.raw_bytes).unwrap();
                // IMMPORTANT THAT IT GOES HERE
                *params.ir_samples.lock().unwrap() = (ir_samples.clone(), ir_sample_rate);

                let processed_ir =
                    process_ir(&ir_samples, ir_sample_rate, sample_rate, &config).unwrap();
                let convolvers = init_convolvers(&processed_ir);

                slot.lock().unwrap().set(FADE_TYPE, FADE_TIME, convolvers);
                *params.ir_data.lock().unwrap() = Some(ir_data);
            }
        })
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
                self.graph.set_sample_rate(buffer_config.sample_rate as f64);
                self.graph.allocate();

                nih_log!("Initialized Convolution.");
                true
            }
            Err(_) => false,
        }
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        Some(Box::new(create_editor(self, async_executor)))
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
