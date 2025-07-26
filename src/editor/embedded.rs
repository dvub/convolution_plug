use include_dir::include_dir;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};

use nih_plug::prelude::AsyncExecutor;
use nih_plug_webview::{
    wry::{
        http::{Request, Response},
        WebViewId,
    },
    WebViewConfig, WebViewEditor, WebViewSource, WebViewState,
};

use crate::{editor::PluginGui, params::PluginParams, ConvolutionPlug};

#[allow(dead_code)]
pub fn embedded_editor(
    state: &Arc<WebViewState>,
    params: &Arc<PluginParams>,
    exec: AsyncExecutor<ConvolutionPlug>,
) -> WebViewEditor {
    let protocol_name = "assets".to_string();

    let rel_config = WebViewConfig {
        title: "Convolution".to_string(),
        source: WebViewSource::CustomProtocol {
            protocol: protocol_name.clone(),
            url: String::new(),
        },
        // tODO: should we change this?
        workdir: PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/webview-workdir"
        )),
    };

    WebViewEditor::new_with_webview(
        PluginGui {
            params: params.clone(),
            executor: exec,
        },
        state,
        rel_config,
        move |builder| {
            builder
                .with_devtools(true)
                .with_custom_protocol(protocol_name.clone(), build_protocol())
        },
    )
}

// TODO: type refactoring is probably pointless
type Res = Response<Cow<'static, [u8]>>;
type Protocol = dyn Fn(WebViewId, Request<Vec<u8>>) -> Res + 'static;

fn build_protocol() -> Box<Protocol> {
    Box::new(move |_id, req: Request<Vec<u8>>| {
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
                .expect("Error constructing response")
        } else {
            panic!("Web asset not found. {file:?}")
        }
    })
}
