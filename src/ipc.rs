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
pub enum Message {
    WindowOpened,
    WindowClosed,
    ParameterUpdate(GUIParams),
    DrawData(f32),
}
#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../convolution-gui/bindings/")]
#[ts(export)]
pub struct GUIParams {
    pub gain: Option<f32>,
    pub dry_wet: Option<f32>,
}

impl From<&Arc<PluginParams>> for GUIParams {
    fn from(params: &Arc<PluginParams>) -> Self {
        GUIParams {
            gain: Some(params.gain.value()),
            dry_wet: Some(params.dry_wet.value()),
        }
    }
}

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
