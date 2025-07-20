use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::IrConfig;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, rename_all = "camelCase", tag = "type", content = "data")]
pub enum Message {
    Init,
    ParameterUpdate(ParameterUpdate),
    IrUpdate(IrData),
    IrConfigUpdate(IrConfig),
    InitResponse(InitResponse),
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, rename_all = "camelCase")]
pub struct InitResponse {
    pub param_map: Vec<String>,
    pub init_params: Vec<ParameterUpdate>,
    pub ir_data: Option<IrData>,
    pub config: IrConfig,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, rename_all = "camelCase")]
pub enum KnobGesture {
    StartDrag,
    StopDrag,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, rename_all = "camelCase")]
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

// TODO: get rid of this and just hold on to the raw bytes for metadata?

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[ts(export, rename_all = "camelCase")]
pub struct IrData {
    pub name: String,
    pub raw_bytes: Vec<u8>,
    pub length_seconds: f32,
    pub num_channels: u16,
    pub sample_rate: f32,
}
