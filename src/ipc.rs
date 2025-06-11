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
    ParameterUpdate(ParameterUpdate),
    DrawData(f32),
}
#[derive(Serialize, Deserialize, TS, Debug)]
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
