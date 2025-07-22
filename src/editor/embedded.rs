use crate::editor::EDITOR_SIZE;

use include_dir::include_dir;

use nih_plug_webview::editor::WebViewEditor;
use nih_plug_webview::http::Response;
use nih_plug_webview::{HTMLSource, WebviewState};
use std::path::Path;
use std::sync::Arc;

// TODO: need better name
pub fn create_embedded_editor(state: Arc<WebviewState>) -> WebViewEditor {
    let protocol_name = "assets";

    #[cfg(target_os = "windows")]
    let url_scheme = format!("http://{protocol_name}.localhost");

    #[cfg(not(target_os = "windows"))]
    let url_scheme = format!("{}://localhost", protocol_name);

    let src = HTMLSource::URL(url_scheme);
    let editor = WebViewEditor::new(src, EDITOR_SIZE, state)
        .with_developer_mode(false)
        .with_custom_protocol(protocol_name.to_string(), move |req| {
            let path = req.uri().path();
            let file = if path == "/" {
                "index.html"
            } else {
                &path[1..]
            };
            // TODO: should we hardcode this or something?
            let dir = include_dir!("$CARGO_MANIFEST_DIR/gui/assets/");

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
    editor
}
