use crate::editor::EDITOR_SIZE;
use crate::params::PluginParams;
use include_dir::include_dir;
use nih_plug_webview::http::Response;
use nih_plug_webview::{HTMLSource, WebViewEditor};
use std::path::Path;
use std::sync::Arc;

#[allow(dead_code)]
pub fn create_bundled_editor(editor: &mut WebViewEditor, params: &Arc<PluginParams>) {
    let protocol_name = "assets";

    #[cfg(target_os = "windows")]
    let url_scheme = format!("http://{protocol_name}.localhost");

    #[cfg(not(target_os = "windows"))]
    let url_scheme = format!("{}://localhost", protocol_name);

    let src = HTMLSource::URL(url_scheme);
    let new_editor = WebViewEditor::new(src, EDITOR_SIZE, params.callback_handler.state.clone());
    // .with_developer_mode(false);

    *editor = new_editor.with_custom_protocol(protocol_name.to_string(), move |req| {
        let path = req.uri().path();
        let file = if path == "/" {
            "index.html"
        } else {
            &path[1..]
        };

        let dir = include_dir!("$CARGO_MANIFEST_DIR/convolution-gui/assets/");

        // mime guess is awesome!
        let mime_type =
            mime_guess::from_ext(Path::new(file).extension().unwrap().to_str().unwrap())
                .first_or_text_plain()
                .to_string();
        if let Some(result_file) = dir.get_file(file) {
            Response::builder()
                .header("content-type", mime_type)
                .header("Access-Control-Allow-Origin", "*")
                .body(result_file.contents().into())
                .map_err(Into::into)
        } else {
            panic!("Web asset not found. {file:?}")
        }
    });
}
