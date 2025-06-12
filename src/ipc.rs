use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use nih_plug::{params::Param, prelude::ParamSetter};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::params::PluginParams;

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Message {
    WindowOpened,
    WindowClosed,
    ParameterUpdate(Vec<ParameterUpdate>),
    DrawData(f32),
}
#[derive(Serialize, Deserialize, TS, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", tag = "parameter", content = "value")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum ParameterUpdate {
    Gain(f32),
    DryWet(f32),
    LowpassEnabled(bool),
    LowpassFreq(f32),
    LowpassQ(f32),
    BellEnabled(bool),
    BellFreq(f32),
    BellQ(f32),
    BellGain(f32),
    HighpassEnabled(bool),
    HighpassFreq(f32),
    HighpassQ(f32),
}
// TODO: is it correct for this to be an impl for Parameter Update?
impl ParameterUpdate {
    pub fn set_plugin_param(&self, setter: &ParamSetter, params: &Arc<PluginParams>) {
        match self {
            ParameterUpdate::Gain(v) => set_param(setter, &params.gain, *v),
            ParameterUpdate::DryWet(v) => set_param(setter, &params.dry_wet, *v),
            ParameterUpdate::LowpassEnabled(v) => set_param(setter, &params.lowpass_enabled, *v),
            ParameterUpdate::LowpassFreq(v) => set_param(setter, &params.lowpass_freq, *v),
            ParameterUpdate::LowpassQ(v) => set_param(setter, &params.lowpass_q, *v),
            ParameterUpdate::BellEnabled(v) => set_param(setter, &params.bell_enabled, *v),
            ParameterUpdate::BellFreq(v) => set_param(setter, &params.bell_freq, *v),
            ParameterUpdate::BellQ(v) => set_param(setter, &params.bell_q, *v),
            ParameterUpdate::BellGain(v) => set_param(setter, &params.bell_gain, *v),
            ParameterUpdate::HighpassEnabled(v) => set_param(setter, &params.highpass_enabled, *v),
            ParameterUpdate::HighpassFreq(v) => set_param(setter, &params.highpass_freq, *v),
            ParameterUpdate::HighpassQ(v) => set_param(setter, &params.highpass_q, *v),
        }
    }
}

fn set_param<P: Param>(setter: &ParamSetter, param: &P, value: P::Plain) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

// TODO: should this be a `impl From`?
pub fn build_param_update_vec(params: &Arc<PluginParams>) -> Vec<ParameterUpdate> {
    vec![
        ParameterUpdate::Gain(params.gain.value()),
        ParameterUpdate::DryWet(params.dry_wet.value()),
        ParameterUpdate::LowpassEnabled(params.lowpass_enabled.value()),
        ParameterUpdate::LowpassFreq(params.lowpass_freq.value()),
        ParameterUpdate::LowpassQ(params.lowpass_q.value()),
        ParameterUpdate::BellEnabled(params.bell_enabled.value()),
        ParameterUpdate::BellFreq(params.bell_freq.value()),
        ParameterUpdate::BellQ(params.bell_q.value()),
        ParameterUpdate::BellGain(params.bell_gain.value()),
        ParameterUpdate::HighpassEnabled(params.highpass_enabled.value()),
        ParameterUpdate::HighpassFreq(params.highpass_freq.value()),
        ParameterUpdate::HighpassQ(params.highpass_q.value()),
    ]
}
