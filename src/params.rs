use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use crossbeam_channel::{Receiver, Sender};
use nih_plug::{prelude::*, util::db_to_gain};
use nih_plug_webview::state::WebviewState;

use crate::editor::ipc::IrData;

// TODO: add smoothers to all params

// TODO: figure out good defaults for filter frequencies
// currently i've got it so that filters do nothing with their default frequencies even if they're enabled
// but maybe it would be more intuitive if the default frequencies had some effect without being crazy

const MIN_FREQ: f32 = 10.0;
const MAX_FREQ: f32 = 22_050.0;

// i got these from playing around with ableton's stock EQs
// they seemed sensible enough to me!
const DEFAULT_Q: f32 = 0.1;
const MIN_Q: f32 = 0.1;
const MAX_Q: f32 = 18.0;

#[derive(Params, Debug)]
pub struct PluginParams {
    // non param stuff
    pub rx: Receiver<usize>,
    pub editor_state: Arc<WebviewState>,

    #[persist = "ir_data"]
    pub ir_data: Mutex<Option<IrData>>,

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

impl Display for PluginParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dry_gain.value())
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        // FIGURE OUT CORRECT LENGTH??
        let (tx, rx) = crossbeam_channel::bounded::<usize>(100);
        let state = WebviewState::new();

        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            dry_gain: FloatParam::new(
                "Dry Gain",
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
            .with_callback(param_update_callback(0, tx.clone(), state.clone())),

            wet_gain: FloatParam::new(
                "Wet Gain",
                db_to_gain(0.0),
                FloatRange::Skewed {
                    min: db_to_gain(-40.0),
                    max: db_to_gain(40.0),
                    factor: FloatRange::gain_skew_factor(-40.0, 40.0),
                },
            )
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_unit(" dB")
            .with_callback(param_update_callback(1, tx.clone(), state.clone())),

            lowpass_enabled: BoolParam::new("Lowpass Enabled", false)
                .with_callback(param_update_callback(2, tx.clone(), state.clone())),

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
            .with_callback(param_update_callback(3, tx.clone(), state.clone())),

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
            .with_callback(param_update_callback(4, tx.clone(), state.clone())),

            highpass_enabled: BoolParam::new("Highpass Enabled", false)
                .with_callback(param_update_callback(5, tx.clone(), state.clone())),
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
            .with_callback(param_update_callback(6, tx.clone(), state.clone())),
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
            .with_callback(param_update_callback(7, tx.clone(), state.clone())),

            bell_enabled: BoolParam::new("Bell Enabled", false)
                .with_callback(param_update_callback(8, tx.clone(), state.clone())),

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
            .with_callback(param_update_callback(9, tx.clone(), state.clone())),
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
            .with_callback(param_update_callback(10, tx.clone(), state.clone())),
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
            .with_callback(param_update_callback(11, tx.clone(), state.clone())),

            // EXTRA GOODIES
            rx,
            editor_state: state,
            ir_data: Mutex::new(None),
        }
    }
}

fn param_update_callback<T>(
    parameter_index: usize,
    tx: Sender<usize>,
    state: Arc<WebviewState>,
) -> Arc<impl Fn(T)>
where
{
    Arc::new(move |_| {
        if state.is_open() {
            tx.try_send(parameter_index)
                .expect("the channel should not be full or try sending if disconnected");
        }
    })
}
