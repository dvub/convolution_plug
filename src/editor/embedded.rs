#![cfg(feature = "embedded-gui")]
use include_dir::include_dir;
use nih_plug_webview::wry::{
    http::{Request, Response},
    WebViewId,
};
use std::{borrow::Cow, path::Path};

// type refactoring is probably pointless?
type Res = Response<Cow<'static, [u8]>>;
type Protocol = dyn Fn(WebViewId, Request<Vec<u8>>) -> Res + 'static;

pub fn build_protocol() -> Box<Protocol> {
    Box::new(move |_id, req: Request<Vec<u8>>| {
        let path = req.uri().path();
        let file = if path == "/" {
            "index.html"
        } else {
            &path[1..]
        };
        // should we hardcode this or something?
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
