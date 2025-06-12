use crate::params::PluginParams;

use nih_plug::{
    params::{BoolParam, FloatParam, Param},
    prelude::ParamSetter,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Parameters {
    Gain,
    DryWet,
    LowpassEnabled,
    LowpassFreq,
    LowpassQ,
    BellEnabled,
    BellFreq,
    BellQ,
    BellGain,
    HighpassEnabled,
    HighpassFreq,
    HighpassQ,
}

// we need this because if we want to return *Params (float, bool, etc.) in a `match`,
// the types would otherwise be mismatched.

// also, when we match on this enum, we can easily convert the generic value
enum ParamWrapper<'a> {
    Float(&'a FloatParam),
    Bool(&'a BoolParam),
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Message<T> {
    WindowOpened,
    WindowClosed,
    ParameterUpdate(ParameterUpdate<T>),
    DrawData(f32),
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct ParameterUpdate<T> {
    pub parameter: Parameters,
    pub value: T,
}

pub fn update_param(
    update: &ParameterUpdate<Value>,
    setter: &ParamSetter,
    params: &Arc<PluginParams>,
) {
    let wrapped_param = match_corresponding(&update.parameter, params);

    let value = &update.value;
    // TODO: fix unwrap()'ing
    match wrapped_param {
        ParamWrapper::Float(p) => set_param(setter, p, value.as_f64().unwrap() as f32),
        ParamWrapper::Bool(p) => set_param(setter, p, value.as_bool().unwrap()),
    }
}

fn match_corresponding<'a>(inp: &Parameters, params: &'a Arc<PluginParams>) -> ParamWrapper<'a> {
    match inp {
        Parameters::Gain => ParamWrapper::Float(&params.gain),
        Parameters::DryWet => ParamWrapper::Float(&params.dry_wet),
        Parameters::LowpassEnabled => ParamWrapper::Bool(&params.lowpass_enabled),
        Parameters::LowpassFreq => ParamWrapper::Float(&params.lowpass_freq),
        Parameters::LowpassQ => ParamWrapper::Float(&params.lowpass_q),
        Parameters::BellEnabled => ParamWrapper::Bool(&params.bell_enabled),
        Parameters::BellFreq => ParamWrapper::Float(&params.bell_freq),
        Parameters::BellQ => ParamWrapper::Float(&params.bell_q),
        Parameters::BellGain => ParamWrapper::Float(&params.bell_gain),
        Parameters::HighpassEnabled => ParamWrapper::Bool(&params.highpass_enabled),
        Parameters::HighpassFreq => ParamWrapper::Float(&params.highpass_freq),
        Parameters::HighpassQ => ParamWrapper::Float(&params.highpass_q),
    }
}

fn set_param<P: Param>(setter: &ParamSetter, param: &P, value: P::Plain) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}
