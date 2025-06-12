use std::sync::Arc;

use nih_plug::nih_log;
use nih_plug_webview::{HTMLSource, WebViewEditor};
use serde_json::json;

use crate::{
    ipc::{build_param_update_vec, Message},
    params::PluginParams,
};

const EDITOR_SIZE: (u32, u32) = (600, 600);

pub fn create_editor(params: &Arc<PluginParams>) -> WebViewEditor {
    let params = params.clone();

    let src = HTMLSource::URL("http://localhost:3000".to_owned());
    let mut editor = WebViewEditor::new(src, EDITOR_SIZE).with_developer_mode(true);

    /*
    #[cfg(not(debug_assertions))]
    {
        use include_dir::include_dir;
        use nih_plug_webview::http::Response;
        use std::path::Path;

        editor = {
            let protocol_name = "assets";

            #[cfg(target_os = "windows")]
            let url_scheme = format!("http://{}.localhost", protocol_name);

            #[cfg(not(target_os = "windows"))]
            let url_scheme = format!("{}://localhost", protocol_name);

            let src = HTMLSource::URL(url_scheme);
            let mut editor = WebViewEditor::new(src, EDITOR_SIZE);

            editor = editor.with_custom_protocol(protocol_name.to_string(), move |req| {
                let path = req.uri().path();
                let file = if path == "/" {
                    "index.html"
                } else {
                    &path[1..]
                };

                let dir = include_dir!("$CARGO_MANIFEST_DIR/gui/assets/");

                // mime guess is awesome!
                let mime_type =
                    mime_guess::from_ext(Path::new(file).extension().unwrap().to_str().unwrap())
                        .first_or_text_plain() // TODO: fix _or_...
                        .to_string();
                if let Some(result_file) = dir.get_file(file) {
                    return Response::builder()
                        .header("content-type", mime_type)
                        .header("Access-Control-Allow-Origin", "*")
                        .body(result_file.contents().into())
                        .map_err(Into::into);
                } else {
                    panic!("Web asset not found. {:?}", file)
                }
            });
            editor
        };
    }*/

    editor = editor.with_event_loop(move |ctx, setter, _window| {
        // handle all incoming messages
        let mut gui_param_updates = Vec::new();

        while let Ok(value) = ctx.next_event() {
            let result = serde_json::from_value::<Message>(value.clone())
                .expect("Error reading message from GUI");

            match result {
                // TODO: add functionality
                Message::WindowOpened => (),
                Message::WindowClosed => (),

                // pretty much the most important one
                Message::ParameterUpdate(updates) => {
                    // this gives us more flexibility in our GUI
                    // we could send individual parameter changes..
                    // OR, we could somehow aggregate them (TODO:)
                    for update in updates {
                        update.set_plugin_param(&setter, &params);
                        gui_param_updates.push(update);
                    }
                }
                // the GUI shouldn't send us draw data, maybe print something but otherwise don't care
                Message::DrawData(_) => {
                    println!("Received draw data from the frontend! (this should not happen)")
                }
            }
        }
        // TODO: figure out performance of this approach
        // (as opposed to communication through callbacks)
        let mut updates_to_send = build_param_update_vec(&params);
        updates_to_send.retain(|u| !gui_param_updates.contains(u));

        // send backend params to our GUI
        // these may be changed by automation, or the user tweaking values in the generic UI
        ctx.send_json(json!(Message::ParameterUpdate(updates_to_send)))
            .expect("Error sending param updates to frontend");
    });

    editor
}
