use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::IRConfig;

// TODO: should structs have a new() method or not?
// currently this is not consistent

// TODO: create specific GUI -> backend enum

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Message {
    Init,
    ParameterUpdate(ParameterUpdate),
    IrUpdate(IrData),
    IrConfigUpdate(IRConfig),
    InitResponse(InitResponse),
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct InitResponse {
    pub param_map: Vec<String>,
    pub init_params: Vec<ParameterUpdate>,
    pub ir_data: Option<IrData>,
    pub config: IRConfig,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum KnobGesture {
    StartDrag,
    StopDrag,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct ParameterUpdate {
    pub parameter_index: usize,
    pub value: f32,
}

impl ParameterUpdate {
    pub fn new(parameter_index: usize, value: f32) -> Self {
        Self {
            parameter_index,
            value,
        }
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct IrData {
    pub name: String,
    pub raw_bytes: Vec<u8>,
    pub length_seconds: f32,
    pub num_channels: u16,
    pub sample_rate: f32,
}
