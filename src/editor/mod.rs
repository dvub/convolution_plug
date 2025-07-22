pub mod embedded;
pub mod event_loop;
pub mod ipc;

#[cfg(not(debug_assertions))]
use crate::editor::embedded::create_embedded_editor;

use crate::{params::PluginParams, ConvolutionPlug};

use event_loop::build_event_loop;

use nih_plug::{editor::Editor, prelude::AsyncExecutor};
use nih_plug_webview::{editor::WebViewEditor, HTMLSource};
use std::sync::Arc;

pub const EDITOR_SIZE: (u32, u32) = (600, 600);

pub fn create_editor(
    params: &Arc<PluginParams>,
    async_executor: AsyncExecutor<ConvolutionPlug>,
) -> Option<Box<dyn Editor>> {
    #[cfg(debug_assertions)]
    let mut editor = create_dev_editor(params);

    #[cfg(not(debug_assertions))]
    let mut editor = create_embedded_editor(params.callback_handler.state.clone());

    editor = editor
        // FROM HEX: #0d100f
        .with_background_color((13, 16, 15, 255))
        .with_event_loop(build_event_loop(params, async_executor));

    Some(Box::new(editor))
}

fn create_dev_editor(params: &Arc<PluginParams>) -> WebViewEditor {
    let dev_src = HTMLSource::URL("http://localhost:3000".to_owned());
    WebViewEditor::new(dev_src, EDITOR_SIZE, params.callback_handler.state.clone())
        .with_developer_mode(true)
}
