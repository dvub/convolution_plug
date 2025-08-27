use nih_plug_webview::Context;
use serde_json::json;

use crate::editor::ipc::Message;

pub const DEFAULT_FREQ_RANGE: (f32, f32) = (20.0, 20_000.0); // hz

pub fn send_message(cx: &mut Context, message: Message) {
    cx.send_message(json!(message).to_string());
}
