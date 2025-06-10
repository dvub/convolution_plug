use nih_plug::{prelude::*, util::db_to_gain};

use crate::ipc::EditorState;

// TODO:
// add highpass and some sort of middle thing for EQ
// other params include... idk

#[derive(Params, Debug)]

pub struct PluginParams {
    pub editor_state: EditorState,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "drywet"]
    pub dry_wet: FloatParam,

    // --- LOWPASS ---
    #[id = "lowpass_enabled"]
    pub lowpass_enabled: BoolParam,

    #[id = "lowpass_cutoff"]
    pub lowpass_freq: FloatParam,

    #[id = "lowpass_q"]
    pub lowpass_q: FloatParam,

    // --- BELL ---
    #[id = "bell_enabled"]
    pub bell_enabled: BoolParam,

    #[id = "bell_freq"]
    pub bell_freq: FloatParam,

    #[id = "bell_q"]
    pub bell_q: FloatParam,

    #[id = "bell_gain"]
    pub bell_gain: FloatParam,

    // --- HIGHPASS ---
    #[id = "highpass_enabled"]
    pub highpass_enabled: BoolParam,

    #[id = "highpass_freq"]
    pub highpass_freq: FloatParam,

    #[id = "highpass_q"]
    pub highpass_q: FloatParam,
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            editor_state: EditorState::default(),
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

            lowpass_enabled: BoolParam::new("Lowpass Enabled", false),

            lowpass_freq: FloatParam::new(
                "Lowpass Frequency",
                22_050.0,
                FloatRange::Linear {
                    min: 10.0,
                    max: 22_050.0,
                },
            ),
            lowpass_q: FloatParam::new(
                "Lowpass Q",
                0.1,
                FloatRange::Linear {
                    min: 0.1,
                    max: 18.0,
                },
            ),

            highpass_enabled: BoolParam::new("Highpass Enabled", false),

            highpass_freq: FloatParam::new(
                "Highpass Frequency",
                22_050.0,
                FloatRange::Linear {
                    min: 10.0,
                    max: 22_050.0,
                },
            ),
            highpass_q: FloatParam::new(
                "Highpass Q",
                0.1,
                FloatRange::Linear {
                    min: 0.1,
                    max: 18.0,
                },
            ),

            bell_enabled: BoolParam::new("Bell Enabled", false),

            bell_freq: FloatParam::new(
                "Bell Frequency",
                22_050.0,
                FloatRange::Linear {
                    min: 10.0,
                    max: 22_050.0,
                },
            ),
            bell_q: FloatParam::new(
                "Bell Q",
                0.1,
                FloatRange::Linear {
                    min: 0.1,
                    max: 18.0,
                },
            ),
            bell_gain: FloatParam::new(
                "Bell Gain",
                0.1,
                FloatRange::Linear {
                    min: 0.1,
                    max: 18.0,
                },
            ),
        }
    }
}
