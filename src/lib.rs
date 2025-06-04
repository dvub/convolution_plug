mod dsp;
mod util;

use fundsp::hacker32::*;
use nih_plug::{prelude::*, util::db_to_gain};
use std::sync::Arc;

use dsp::{convolver, dry_wet};
use util::{read_samples_from_file, rms_normalize};

use crate::dsp::{gain, lowpass_cutoff};

type StereoBuffer = BufferArray<U2>;

struct ConvolutionPlug {
    params: Arc<ConvolutionPlugParams>,

    graph: Box<dyn AudioUnit>,

    input_buffer: StereoBuffer,
    output_buffer: StereoBuffer,
}

#[derive(Params)]
struct ConvolutionPlugParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "drywet"]
    pub dry_wet: FloatParam,

    #[id = "lowcutoff"]
    pub lowpass_cutoff: FloatParam,

    #[id = "lowcutq"]
    pub lowpass_q: FloatParam,
}

impl Default for ConvolutionPlug {
    fn default() -> Self {
        // TODO: how else can you make this as cheap as possible?
        let graph = pass() | pass();

        Self {
            params: Arc::new(ConvolutionPlugParams::default()),

            graph: Box::new(graph),
            input_buffer: BufferArray::<U2>::new(),
            output_buffer: BufferArray::<U2>::new(),
        }
    }
}

impl Default for ConvolutionPlugParams {
    fn default() -> Self {
        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            gain: FloatParam::new(
                "Gain",
                db_to_gain(0.0),
                FloatRange::Skewed {
                    min: db_to_gain(-30.0),
                    max: db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            dry_wet: FloatParam::new("Dry/Wet", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit("%"),

            lowpass_cutoff: FloatParam::new(
                "Lowpass Cutoff",
                11_000.0,
                FloatRange::Linear {
                    min: 10.0,
                    max: 22_000.0,
                },
            ),
            lowpass_q: FloatParam::new(
                "Lowpass Cutoff",
                5.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 10.0,
                },
            ),
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
        let path = "D:\\projects\\rust\\convolution_plug\\test_irs\\large.wav";
        // let path = "C:\\Users\\Kaya\\Documents\\projects\\convolution_plug\\test_irs\\large.wav";

        let mut ir_samples = read_samples_from_file(path);
        rms_normalize(&mut ir_samples, -48.0);

        let p = &self.params;

        let mono_wet = (convolver(&ir_samples) | lowpass_cutoff::<U1>(p) | dsp::lowpass_q::<U1>(p))
            >> lowpass();

        let wet = mono_wet * dry_wet::<U1>(p);
        let dry = pass() * (1.0 - dry_wet::<U1>(p));

        let mixed = wet & dry;

        let graph = mixed >> split::<U2>();

        self.graph = Box::new(graph);

        nih_log!("Initialized Convolution");

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
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
