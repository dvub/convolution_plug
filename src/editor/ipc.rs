use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::processing::config::IrProcessingConfig;

// NOTE: im not exactly sure why, but if we use
// #[ts(export, rename_all = ...)]
// instead of serde, things do not work

// unfortunately this prevents a lot of this code from looking cleaner
#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export)]
pub enum Message {
    Init,
    ParameterUpdate(ParameterUpdate),
    IrUpdate(IrData),
    IrConfigUpdate(IrProcessingConfig),
    InitResponse(InitResponse),
    Resize { width: f64, height: f64 },
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct InitResponse {
    pub param_map: Vec<String>,
    pub init_params: Vec<ParameterUpdate>,
    pub ir_data: Option<IrData>,
    pub config: IrProcessingConfig,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ParameterUpdate {
    pub parameter_index: usize,
    pub value: f32,
}

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct IrData {
    pub name: String,
    pub raw_bytes: Vec<u8>,
}
