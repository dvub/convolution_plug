mod bundled;
pub mod event_loop;
pub mod ipc;

use crate::ConvolutionPlug;
use event_loop::build_event_loop;

use nih_plug::prelude::AsyncExecutor;
use nih_plug_webview::{HTMLSource, WebViewEditor};

pub const EDITOR_SIZE: (u32, u32) = (600, 600);

pub fn create_editor(
    plugin: &ConvolutionPlug,
    async_executor: AsyncExecutor<ConvolutionPlug>,
) -> WebViewEditor {
    let params = plugin.params.clone();

    let dev_src = HTMLSource::URL("http://localhost:3000".to_owned());

    #[allow(unused_mut)]
    let mut editor = WebViewEditor::new(dev_src, EDITOR_SIZE, params.editor_state.clone())
        .with_developer_mode(true)
        // FROM HEX: #0d100f
        .with_background_color((13, 16, 15, 255));

    #[cfg(not(debug_assertions))]
    bundled::create_bundled_editor(&mut editor, &params);

    editor.with_event_loop(build_event_loop(plugin, async_executor))
}
