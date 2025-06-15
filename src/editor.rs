use std::sync::Arc;

use nih_plug::{
    params::Params,
    prelude::{ParamPtr, ParamSetter},
};
use nih_plug_webview::{HTMLSource, WebViewEditor};
use serde_json::json;

use crate::{
    ipc::{Message, ParameterUpdate},
    params::PluginParams,
};

use itertools::Itertools;

type ParamMap = Vec<(String, ParamPtr, String)>;

const EDITOR_SIZE: (u32, u32) = (600, 600);

// TODO:
// figure out where to correctly use unsafe keyword for param stuff

// TODO: fix nesting issues
pub fn create_editor(params: &Arc<PluginParams>) -> WebViewEditor {
    let params = params.clone();
    let map = params.param_map();
    let param_update_rx = params.rx.clone();

    println!("PARAM MAP: {:?}", map);

    let src = HTMLSource::URL("http://localhost:3000");
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
        let mut gui_updates = Vec::new();

        // --- GUI -> BACKEND COMMUNICATION ---
        while let Ok(value) = ctx.next_event() {
            let result = serde_json::from_value::<Message>(value.clone())
                .expect("Error reading message from GUI");

            match result {
                Message::Init => unsafe {
                    let map = params.param_map();
                    for entry in map {
                        ctx.send_json(json!(Message::ParameterUpdate(ParameterUpdate {
                            parameter_id: entry.0,
                            // TODO: change this to get the actual corresponding float or bool param
                            // then get the actual f32 OR bool out of that
                            value: entry.1.modulated_plain_value().to_string()
                        })));
                    }
                },
                // pretty much the most important one
                Message::ParameterUpdate(update) => unsafe {
                    match_and_update_param(&update, &setter, &map);
                    gui_updates.push(update.parameter_id)
                },
                // baseview has bugs on windows
                // once fixed this can be implemented
                Message::Resize { .. } => todo!(),
            }
        }
        // --- BACKEND -> GUI COMMUNICATION ---

        // for each iteration of this event loop, we only really need to send one update for each parameter
        // therefore, we use unique() to remove duplicate parameter IDs
        for param_id in param_update_rx.try_iter().unique() {
            // if a parameter update comes from GUI, we don't want to send an old (-ish) version of the same parameter to the GUI
            if gui_updates.contains(&param_id) {
                continue;
            }

            // now that we know we REALLY want to send this parameter update to the GUI
            // we do so here
            unsafe {
                let update = ParameterUpdate {
                    parameter_id: param_id.clone(),
                    value: get_normalized_param_value(param_id, &map),
                };
                let message = Message::ParameterUpdate(update);
                ctx.send_json(json!(message));
            }
        }
    });

    editor
}

// TODO: overhaul error handling for this function
// (due to unwrapping the parse())

unsafe fn match_and_update_param(update: &ParameterUpdate, setter: &ParamSetter, map: &ParamMap) {
    let value = update.value.as_str();
    let id = update.parameter_id.as_str();

    let ptr = get_ptr(id.to_owned(), map);
    raw_set_param(setter, ptr, value.parse().unwrap());
}

unsafe fn get_normalized_param_value(id: String, map: &ParamMap) -> String {
    let ptr = get_ptr(id, map);
    ptr.modulated_normalized_value().to_string()

    // OLD APPROACH
    /*
    match ptr {
        ParamPtr::FloatParam(p) => {
            let float_param = &*p;
            float_param.value().to_string()
        }
        ParamPtr::BoolParam(p) => {
            let bool_param = &*p;
            bool::to_string(&bool_param.value())
        }
        // not implemented (yet)
        ParamPtr::IntParam(_) => todo!(),
        ParamPtr::EnumParam(_) => todo!(),
    }
    */
}

fn get_ptr(id: String, map: &ParamMap) -> ParamPtr {
    map.iter()
        .find(|(param_id, _, _)| id == *param_id)
        .unwrap_or_else(|| panic!("Couldn't find a parameter with ID {}", id))
        .1
}

// TODO: is there a better way to do this??
unsafe fn raw_set_param(setter: &ParamSetter, param: ParamPtr, normalized: f32) {
    setter.raw_context.raw_begin_set_parameter(param);
    setter
        .raw_context
        .raw_set_parameter_normalized(param, normalized);
    setter.raw_context.raw_end_set_parameter(param);
}
