use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Message {
    Init,
    Resize {
        width: u32,
        height: u32,
    },
    ParameterUpdate(ParameterUpdate),
    IrUpdate(IrData),
    KnobGesture {
        parameter_id: String,
        gesture: KnobGesture,
    },
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
    pub parameter_id: String,
    pub value: f32,
}

impl ParameterUpdate {
    pub fn new(parameter_id: String, value: f32) -> Self {
        Self {
            parameter_id,
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
