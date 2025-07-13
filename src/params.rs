use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crossbeam_channel::{Receiver, Sender};

use nih_plug::{prelude::*, util::db_to_gain};
use nih_plug_webview::state::WebviewState;

use crate::{config::PluginConfig, editor::ipc::IrData};

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

const SMOOTHER: SmoothingStyle = SmoothingStyle::Linear(0.5);

const DEFAULT_WET_GAIN: f32 = -15.0;
const DEFAULT_DRY_GAIN: f32 = -10.0;

#[derive(Params, Debug)]
pub struct PluginParams {
    // non param stuff
    pub rx: Receiver<usize>,
    pub editor_state: Arc<WebviewState>,
    pub are_params_dragging: Vec<Arc<AtomicBool>>,

    #[persist = "config"]
    pub config: Mutex<PluginConfig>,

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
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()),

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
            .with_smoother(SMOOTHER)
            .with_callback(callback_handler.create_callback()),

            lowpass_enabled: BoolParam::new("Lowpass Enabled", false)
                .with_callback(callback_handler.create_callback()),

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
            .with_callback(callback_handler.create_callback()),

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
            .with_callback(callback_handler.create_callback()),

            highpass_enabled: BoolParam::new("Highpass Enabled", false)
                .with_callback(callback_handler.create_callback()),
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
            .with_callback(callback_handler.create_callback()),
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
            .with_callback(callback_handler.create_callback()),

            bell_enabled: BoolParam::new("Bell Enabled", false)
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

            // EXTRA GOODIES
            // TODO: should we just have one callback_handler field?
            rx: callback_handler.rx,
            editor_state: callback_handler.state,
            are_params_dragging: callback_handler.are_params_dragging,

            // persistent
            ir_data: Mutex::new(None),
            config: Mutex::new(PluginConfig::default()),
        }
    }
}

struct CallbackHandler {
    counter: usize,
    state: Arc<WebviewState>,
    tx: Sender<usize>,
    rx: Receiver<usize>,
    are_params_dragging: Vec<Arc<AtomicBool>>,
}
impl Default for CallbackHandler {
    fn default() -> Self {
        // TODO: figure out proper size
        let (tx, rx) = crossbeam_channel::bounded::<usize>(128);
        let state = WebviewState::new();
        let params_dragging = Vec::new();

        Self {
            counter: 0,
            state,
            tx,
            rx,
            are_params_dragging: params_dragging,
        }
    }
}

impl CallbackHandler {
    pub fn create_callback<T>(&mut self) -> Arc<impl Fn(T)> {
        let state = self.state.clone();
        let tx = self.tx.clone();
        let parameter_index = self.counter;

        let is_param_dragging = Arc::new(AtomicBool::new(false));

        self.are_params_dragging.push(is_param_dragging.clone());

        self.counter += 1;

        Arc::new(move |_| {
            if !state.is_open() || is_param_dragging.load(Ordering::Relaxed) {
                return;
            }

            tx.try_send(parameter_index)
                .expect("the channel should not be full or try sending if disconnected");
        })
    }
}
