use std::{fmt::Display, sync::Arc};

use crossbeam_channel::{Receiver, Sender};
use nih_plug::{prelude::*, util::db_to_gain};

// TODO:
// add highpass and some sort of middle thing for EQ
// other params include... idk

#[derive(Params, Debug)]
pub struct PluginParams {
    pub rx: Receiver<String>,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "dry_wet"]
    pub dry_wet: FloatParam,

    // --- LOWPASS ---
    #[id = "lowpass_enabled"]
    pub lowpass_enabled: BoolParam,

    #[id = "lowpass_freq"]
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

impl Display for PluginParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.gain.value())
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded::<String>();

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
            // .with_smoother(SmoothingStyle::Logarithmic(50.0))
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_unit(" dB")
            .with_callback(generate_callback(String::from("gain"), tx.clone())),
            dry_wet: FloatParam::new("Dry/Wet", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit("%")
                .with_callback(generate_callback(String::from("dry_wet"), tx.clone())),

            lowpass_enabled: BoolParam::new("Lowpass Enabled", false).with_callback(
                generate_callback(String::from("lowpass_enabled"), tx.clone()),
            ),

            lowpass_freq: FloatParam::new(
                "Lowpass Frequency",
                22_050.0,
                FloatRange::Skewed {
                    min: 10.0,
                    max: 22_050.0,
                    factor: FloatRange::skew_factor(-2.5),
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_callback(generate_callback(String::from("lowpass_freq"), tx.clone())),

            lowpass_q: FloatParam::new(
                "Lowpass Q",
                0.1,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 18.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_callback(generate_callback(String::from("lowpass_q"), tx.clone())),

            highpass_enabled: BoolParam::new("Highpass Enabled", false).with_callback(
                generate_callback(String::from("highpass_enabled"), tx.clone()),
            ),
            highpass_freq: FloatParam::new(
                "Highpass Frequency",
                22_050.0,
                FloatRange::Skewed {
                    min: 10.0,
                    max: 22_050.0,
                    factor: FloatRange::skew_factor(-2.5),
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_callback(generate_callback(String::from("highpass_freq"), tx.clone())),
            highpass_q: FloatParam::new(
                "Highpass Q",
                0.1,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 18.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_callback(generate_callback(String::from("highpass_q"), tx.clone())),

            bell_enabled: BoolParam::new("Bell Enabled", false)
                .with_callback(generate_callback(String::from("bell_enabled"), tx.clone())),

            bell_freq: FloatParam::new(
                "Bell Frequency",
                22_050.0,
                FloatRange::Skewed {
                    min: 10.0,
                    max: 22_050.0,
                    factor: FloatRange::skew_factor(-2.5),
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_callback(generate_callback(String::from("bell_freq"), tx.clone())),
            bell_q: FloatParam::new(
                "Bell Q",
                0.1,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 18.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_callback(generate_callback(String::from("bell_q"), tx.clone())),
            bell_gain: FloatParam::new(
                "Bell Gain",
                db_to_gain(0.0),
                FloatRange::Skewed {
                    min: db_to_gain(-15.0),
                    max: db_to_gain(15.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_unit(" dB")
            .with_callback(generate_callback(String::from("bell_gain"), tx.clone())),
            rx,
        }
    }
}

// TODO: figure out String or &str
fn generate_callback<T>(parameter: String, tx: Sender<String>) -> Arc<impl Fn(T)>
where
{
    Arc::new(move |_| {
        // TODO: shoud we handle errors?
        let _ = tx.try_send(parameter.clone());
    })
}
