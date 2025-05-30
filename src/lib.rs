use fft_convolver::FFTConvolver;
use nih_plug::{prelude::*, util::db_to_gain};

use std::sync::Arc;

const CONVOLVER_BLOCK_SIZE: usize = 256;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct ConvolutionPlug {
    scratch: Vec<f32>,

    // TODO: not sure if this needs to be here
    impulse_response: Vec<f32>,

    convolvers: [FFTConvolver<f32>; 2],

    params: Arc<ConvolutionPlugParams>,
}

#[derive(Params)]
struct ConvolutionPlugParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for ConvolutionPlug {
    fn default() -> Self {
        Self {
            impulse_response: Vec::new(),
            convolvers: [FFTConvolver::default(), FFTConvolver::default()],

            scratch: Vec::new(),
            params: Arc::new(ConvolutionPlugParams::default()),
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
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
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
        // i have no idea if this will work
        let max_buf_len = buffer_config.max_buffer_size;
        self.scratch = vec![0.0; max_buf_len as usize];

        let mut ir_samples = read_samples_from_file(
            "C:\\Users\\Kaya\\Documents\\projects\\convolution_plug\\test_irs\\medium.wav",
        );

        rms_normalize(&mut ir_samples, -48.0);

        self.impulse_response = ir_samples;

        self.convolvers.iter_mut().for_each(|convolver| {
            convolver
                .init(CONVOLVER_BLOCK_SIZE, &self.impulse_response)
                .unwrap();
        });

        nih_log!("Initialized Convolution");
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
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
        for (i, channel) in buffer.as_slice().iter_mut().enumerate() {
            let scratch = &mut self.scratch[..channel.len()];
            scratch.clone_from_slice(channel);

            self.convolvers[i].process(scratch, channel).unwrap();
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

fn read_samples_from_file(path: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(path).unwrap();

    let bit_depth = reader.spec().bits_per_sample as u32;

    let max_amplitude = 2_i32.pow(bit_depth - 1) as f32;
    reader
        .samples::<i32>()
        .map(|s| s.unwrap_or(0) as f32 / max_amplitude)
        .collect()
}

// first attempt was peak normalization, didn't work very well for a variety of irs
// https://hackaudio.com/tutorial-courses/learn-audio-programming-table-of-contents/digital-signal-processing/amplitude/rms-normalization/

fn rms_normalize(input: &mut [f32], level: f32) {
    let n = input.len() as f32;
    let r = db_to_gain(level);

    let squared_sum = input.iter().map(|x| x * x).sum::<f32>();

    let a = ((n * r.powi(2)) / squared_sum).sqrt();
    println!("Normalizing by factor: {}", a);

    input.iter_mut().for_each(|x| *x *= a);
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::PI, fs::remove_file};

    use nih_plug::util::gain_to_db;

    use crate::rms_normalize;

    use super::read_samples_from_file;

    // test function
    // this writes a file AND returns an array of the samples
    // then the read function can be tested by comparing samples
    fn write_test_file(name: &str) -> Vec<f32> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        // better to keep this short - easier to inspect for testing
        let len = 100;

        let mut writer = hound::WavWriter::create(name, spec).unwrap();
        let mut samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            samples.push(sample);
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
        writer.finalize().unwrap();

        samples
    }

    // TODO: make this test pass without manually checking
    #[test]
    fn test_read_write() {
        // TODO: use better name and proper temp directory
        let file_name = "sine.wav";

        let samples = write_test_file(file_name);
        let other = read_samples_from_file(file_name);

        // this might be horrible
        remove_file(file_name).unwrap();

        assert_eq!(samples, other);
    }

    // TODO: make this stupid test pass
    #[test]
    fn test_normalize() {
        let mut samples = read_samples_from_file("test_irs\\vsmall.wav");

        let desired_rms = -18.0f32;
        rms_normalize(&mut samples, desired_rms);

        let n = samples.len() as f32;
        let new_rms = (samples.iter().map(|x| x.powi(2)).sum::<f32>() / n).sqrt();

        assert_eq!(gain_to_db(new_rms), desired_rms);
    }
}
