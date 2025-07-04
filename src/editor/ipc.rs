use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Message {
    Init,
    Resize { width: u32, height: u32 },
    ParameterUpdate(ParameterUpdate),
    IrUpdate(IrData),
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct ParameterUpdate {
    pub parameter_id: String,
    pub value: f32,
}
#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct IrData {
    pub name: String,
    pub raw_bytes: Vec<u8>,
}
