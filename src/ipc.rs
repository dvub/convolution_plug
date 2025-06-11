use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::params::PluginParams;

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub enum Message<T> {
    WindowOpened,
    WindowClosed,
    ParameterUpdates(Vec<ParameterUpdate<T>>),
    DrawData(f32),
}
#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct ParameterUpdate<T> {
    paramter: Parameters,
    value: T,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
// TODO: macro or something to automatically generate this
enum Parameters {
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

/*

// TODO:
// move this to nih-plug-webview itself
// refer to nih plug vizia docs or something
#[derive(Debug, Default)]
pub struct EditorState {
    open: AtomicBool,
}
impl EditorState {
    // TODO:
    // figure out corect Ordering
    pub fn set_open(&self) {
        self.open.store(true, Ordering::Relaxed);
    }

    pub fn set_closed(&self) {
        self.open.store(false, Ordering::Relaxed);
    }

    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Relaxed)
    }
}

*/
