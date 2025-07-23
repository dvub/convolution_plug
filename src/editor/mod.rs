use std::{path::PathBuf, sync::Arc};

use nih_plug::editor::Editor;
use nih_plug_webview::{
    Context, EditorHandler, Message, WebViewConfig, WebViewEditor, WebViewSource, WebViewState,
};

pub struct PluginGui {}

impl PluginGui {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(state: &Arc<WebViewState>) -> Option<Box<dyn Editor>> {
        let config = WebViewConfig {
            title: String::from("My Plugin"),
            source: WebViewSource::URL(String::from("http://localhost:3000")),
            workdir: PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/target/webview-workdir"
            )),
        };
        let editor = WebViewEditor::new(PluginGui {}, state, config);

        Some(Box::new(editor))
    }
}

impl EditorHandler for PluginGui {
    fn init(&mut self, cx: &mut Context) {
        cx.send_message(Message::Text(String::from("This is an init message")));
    }

    fn on_frame(&mut self, cx: &mut Context) {
              cx.send_message(Message::Text(String::from("This is a frame message")));
    }

    fn on_message(
        &mut self,
        _: &dyn Fn(nih_plug_webview::Message),
        message: nih_plug_webview::Message,
    ) {
        todo!()
    }
}
