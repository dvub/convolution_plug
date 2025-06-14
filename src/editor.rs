use std::sync::Arc;

use nih_plug::{
    params::{Param, Params},
    prelude::{ParamPtr, ParamSetter},
};
use nih_plug_webview::{HTMLSource, WebViewEditor};
use serde_json::json;

use crate::{
    ipc::{Message, ParameterUpdate},
    params::PluginParams,
};

const EDITOR_SIZE: (u32, u32) = (600, 600);

// TODO: fix nesting issues
pub fn create_editor(params: &Arc<PluginParams>) -> WebViewEditor {
    let params = params.clone();

    let param_rx_clone = params.rx.clone();
    println!("PARAM MAP: {:?}", params.param_map());

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

        // handle GUI -> backend messages
        while let Ok(value) = ctx.next_event() {
            let result = serde_json::from_value::<Message>(value.clone())
                .expect("Error reading message from GUI");

            // TODO:
            // refactor
            match result {
                Message::Init => unsafe {
                    let map = params.param_map();
                    for entry in map {
                        ctx.send_json(json!(Message::ParameterUpdate(ParameterUpdate {
                            parameter_id: entry.0,
                            // TODO: make sure this is the value we really want
                            value: entry.1.modulated_plain_value().to_string()
                        })));
                    }
                },
                // pretty much the most important one
                Message::ParameterUpdate(update) => {
                    match_and_update_param(&update, &setter, &params);
                    gui_updates.push(update)
                }
                // baseview has bugs on windows
                // once fixed this can be implemented
                Message::Resize { .. } => todo!(),
            }
        }
        // send param updates backend -> GUI
        while let Ok(param_update) = param_rx_clone.try_recv() {
            if gui_updates
                .iter()
                .any(|p| p.parameter_id == param_update.parameter_id)
            {
                continue;
            }

            ctx.send_json(json!(Message::ParameterUpdate(param_update)));
            //println!("Sent parameter update to GUI: {:?}", param_update);
        }
    });

    editor
}

// TODO: overhaul error handling for this function
// TODO: mark function as unsafe?
fn match_and_update_param(
    update: &ParameterUpdate,
    setter: &ParamSetter,
    params: &Arc<PluginParams>,
) {
    let value = update.value.as_str();
    let id = update.parameter_id.as_str();

    let map = params.param_map();
    let ptr = map
        .iter()
        .find(|(param_id, _, _)| id == param_id)
        .unwrap_or_else(|| panic!("Couldn't find a parameter with ID {}", id))
        .1;

    // "Dereferencing the pointers stored in the values is only valid as long as [the param_map() object] is valid."
    // so we should be fine to dereference these pointers.
    // this also allows rust to smartly parse the incoming value (which is a string)
    unsafe {
        match ptr {
            ParamPtr::FloatParam(p) => {
                let float_param = &*p;
                set_param(setter, float_param, value.parse::<f32>().unwrap());
            }
            ParamPtr::BoolParam(p) => {
                let bool_param = &*p;
                set_param(setter, bool_param, value.parse::<bool>().unwrap());
            }
            // not implemented (yet)
            ParamPtr::IntParam(_) => todo!(),
            ParamPtr::EnumParam(_) => todo!(),
        }
    }
}

// TODO: is there a better way to do this
fn set_param<P: Param>(setter: &ParamSetter, param: &P, value: P::Plain) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

/*     match id {
    "gain" => set_param(setter, &params.gain, value.parse().unwrap()),
    "dry_wet" => set_param(setter, &params.dry_wet, value.parse().unwrap()),

    // LOWPASS
    "lowpass_enabled" => set_param(setter, &params.lowpass_enabled, value.parse().unwrap()),
    "lowpass_freq" => set_param(setter, &params.lowpass_freq, value.parse().unwrap()),
    "lowpass_q" => set_param(setter, &params.lowpass_q, value.parse().unwrap()),
    // BELL
    "bell_enabled" => set_param(setter, &params.bell_enabled, value.parse().unwrap()),
    "bell_freq" => set_param(setter, &params.bell_freq, value.parse().unwrap()),
    "bell_q" => set_param(setter, &params.bell_q, value.parse().unwrap()),
    "bell_gain" => set_param(setter, &params.bell_gain, value.parse().unwrap()),
    // HP
    "highpass_enabled" => set_param(setter, &params.highpass_enabled, value.parse().unwrap()),
    "highpass_freq" => set_param(setter, &params.highpass_freq, value.parse().unwrap()),
    "highpass_q" => set_param(setter, &params.highpass_q, value.parse().unwrap()),

    &_ => nih_log!("Receiving unknown parameter ID"),
}
*/
