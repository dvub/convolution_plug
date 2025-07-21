use std::sync::Mutex;

use nih_plug::{prelude::*, util::db_to_gain};

use crate::{
    callbacks::CallbackHandler, editor::ipc::IrData, processing::config::IrProcessingConfig,
};

// should we use different default filter frequencies?
// currently i've got i t so that filters do nothing with their default frequencies even if they're enabled
// but maybe it would be more intuitive if the default frequencies had some effect without being crazy

const MIN_FREQ: f32 = 10.0;
const MAX_FREQ: f32 = 22_050.0;

// i got these from playing around with ableton's stock EQs
// they seemed sensible enough to me!
const DEFAULT_Q: f32 = 0.1;
const MIN_Q: f32 = 0.1;
const MAX_Q: f32 = 18.0;

const SMOOTHER: SmoothingStyle = SmoothingStyle::Linear(50.0);
const GAIN_SMOOTHER: SmoothingStyle = SmoothingStyle::Logarithmic(50.0);

pub const DEFAULT_WET_GAIN: f32 = -15.0;
pub const DEFAULT_DRY_GAIN: f32 = -10.0;

#[derive(Params, Debug)]
pub struct PluginParams {
    // non param stuff
    pub callback_handler: CallbackHandler,

    #[persist = "config"]
    pub ir_config: Mutex<IrProcessingConfig>,

    #[persist = "ir_data"]
    pub ir_data: Mutex<Option<IrData>>,

    #[persist = "ir_samples"]
    pub ir_samples: Mutex<(Vec<Vec<f32>>, f32)>,

    // --- actual param stuff ---
    #[id = "dry_gain"]
    pub dry_gain: FloatParam,

    #[id = "wet_gain"]
    pub wet_gain: FloatParam,

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

impl Default for PluginParams {
    fn default() -> Self {
        let mut callback_handler = CallbackHandler::default();

        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            dry_gain: FloatParam::new(
                "Dry Gain",
                db_to_gain(DEFAULT_DRY_GAIN),
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
            .with_smoother(GAIN_SMOOTHER)
            .with_callback(callback_handler.create_callback()), //0

            wet_gain: FloatParam::new(
                "Wet Gain",
                db_to_gain(DEFAULT_WET_GAIN),
                FloatRange::Skewed {
                    min: db_to_gain(-40.0),
                    max: db_to_gain(40.0),
                    factor: FloatRange::gain_skew_factor(-40.0, 40.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_unit(" dB")
            .with_smoother(GAIN_SMOOTHER)
            .with_callback(callback_handler.create_callback()), // 1

            lowpass_enabled: BoolParam::new("Lowpass Enabled", false)
                .with_callback(callback_handler.create_callback()), //  2

            lowpass_freq: FloatParam::new(
                "Lowpass Frequency",
                MAX_FREQ,
                FloatRange::Skewed {
                    min: MIN_FREQ,
                    max: MAX_FREQ,
                    factor: FloatRange::skew_factor(-2.5),
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()), // 3

            lowpass_q: FloatParam::new(
                "Lowpass Q",
                DEFAULT_Q,
                FloatRange::Skewed {
                    min: MIN_Q,
                    max: MAX_Q,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()), // 4

            bell_enabled: BoolParam::new("Bell Enabled", false) // 8
                .with_callback(callback_handler.create_callback()),

            bell_freq: FloatParam::new(
                "Bell Frequency",
                MIN_FREQ,
                FloatRange::Skewed {
                    min: MIN_FREQ,
                    max: MAX_FREQ,
                    factor: FloatRange::skew_factor(-2.5),
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()),
            bell_q: FloatParam::new(
                "Bell Q",
                DEFAULT_Q,
                FloatRange::Skewed {
                    min: MIN_Q,
                    max: MAX_Q,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()),
            bell_gain: FloatParam::new(
                "Bell Gain",
                db_to_gain(0.0),
                FloatRange::Skewed {
                    min: db_to_gain(-15.0),
                    max: db_to_gain(15.0),
                    factor: FloatRange::gain_skew_factor(-15.0, 15.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_unit(" dB")
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()),

            highpass_enabled: BoolParam::new("Highpass Enabled", false)
                .with_callback(callback_handler.create_callback()), // 5
            highpass_freq: FloatParam::new(
                "Highpass Frequency",
                MIN_FREQ,
                FloatRange::Skewed {
                    min: MIN_FREQ,
                    max: MAX_FREQ,
                    factor: FloatRange::skew_factor(-2.5),
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz())
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()), // 6
            highpass_q: FloatParam::new(
                "Highpass Q",
                DEFAULT_Q,
                FloatRange::Skewed {
                    min: MIN_Q,
                    max: MAX_Q,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()), // 7

            // EXTRA GOODIES
            callback_handler,

            // persistent
            ir_data: Mutex::new(None),
            ir_config: Mutex::new(IrProcessingConfig::default()),
            ir_samples: Mutex::new((Vec::new(), 0.0)),
        }
    }
}
