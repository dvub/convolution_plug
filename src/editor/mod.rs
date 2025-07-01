mod ipc;

use fundsp::hacker32::*;
use ipc::{Message, ParameterUpdate};

use crate::{
    dsp::convolve::convolver,
    util::{decode_samples, rms_normalize},
    ConvolutionPlug,
};

use itertools::Itertools;
use nih_plug::{
    params::Params,
    prelude::{ParamPtr, ParamSetter},
};
use nih_plug_webview::{HTMLSource, WebViewEditor};

use serde_json::json;

use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

type ParamMap = Vec<(String, ParamPtr, String)>;

const EDITOR_SIZE: (u32, u32) = (600, 600);

// TODO:
// figure out where to correctly use unsafe keyword for param stuff

// TODO: fix nesting issues
pub fn create_editor(plugin: &mut ConvolutionPlug) -> WebViewEditor {
    let params = plugin.params.clone();
    let param_map = params.param_map();
    let param_update_rx = params.rx.clone();
    let config = plugin.config.clone();
    let sample_rate = plugin.sample_rate;
    let slot = plugin.slot.clone();

    println!("PARAM MAP: {:?}", param_map);

    let src = HTMLSource::URL("http://localhost:3000");

    let mut editor =
        WebViewEditor::new(src, EDITOR_SIZE, params.editor_state.clone()).with_developer_mode(true);

    /*
    //#[cfg(not(debug_assertions))]
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

            let src = HTMLSource::URL(&url_scheme);
            let mut editor = WebViewEditor::new(src, EDITOR_SIZE, params.editor_state.clone());

            editor = editor.with_custom_protocol(protocol_name.to_string(), move |req| {
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
                    for param_ptr in &param_map {
                        let param_update = ParameterUpdate {
                            parameter_id: param_ptr.0.clone(),
                            value: param_ptr.1.modulated_normalized_value(),
                        };
                        let message = Message::ParameterUpdate(param_update);

                        ctx.send_json(json!(message));
                    }
                },

                Message::ParameterUpdate(update) => unsafe {
                    match_and_update_param(&update, &setter, &param_map);
                    gui_updates.push(update.parameter_id)
                },
                // TODO: improve error handling
                // GUI thread doesn't have to be real-time
                // so we're gonna do a buunch of non real-time stuff here
                Message::SlotUpdate(ir_file_bytes) => {
                    let (ir_samples, ir_sample_rate) = decode_samples(ir_file_bytes.as_slice());

                    // TODO: refactor this quite a lot
                    let resampling_params = SincInterpolationParameters {
                        sinc_len: 512,
                        f_cutoff: 5.0,
                        interpolation: SincInterpolationType::Cubic,
                        oversampling_factor: 512,
                        window: WindowFunction::Hann,
                    };

                    let mut resampler = SincFixedIn::<f32>::new(
                        sample_rate as f64 / ir_sample_rate as f64,
                        10.0,
                        resampling_params,
                        1024,
                        1,
                    )
                    .unwrap();

                    let mut resampled_ir = resampler.process(&[ir_samples], None).unwrap();
                    let res = &mut resampled_ir[0];

                    if config.normalize_irs {
                        rms_normalize(res, config.normalization_level);
                    }

                    // 2. update our convolver via frontend
                    let new_unit = Box::new(convolver(res) | convolver(res));

                    // in our case, i think the fading *type* is such a small detail that it's okay not to expose it as an option in any way
                    slot.lock()
                        .unwrap()
                        .set(Fade::Smooth, config.fade_time, new_unit);

                    // 3. make this particular IR persistent
                    // Params require the persistent field to be a Mutex<Vec<T>> instead of just a Vec
                    // so we should lock the mutex and update it here
                    // (instead of the audio thread)
                    // let mut lock = params.persistent_ir_samples.lock().unwrap();
                    // *lock = Some(ir_samples);
                }

                // baseview has bugs on windows
                // once fixed this can be implemented

                // i should do that
                Message::Resize { .. } => todo!(),
            }
        }
        // --- BACKEND -> GUI COMMUNICATION ---

        // TODO:
        // write test for this behavior

        // for each iteration of this event loop, we only really need to send one update for each parameter
        // therefore, we use unique() to remove duplicate parameter IDs
        for param_index in param_update_rx.try_iter().unique() {
            let param_id = &param_map[param_index].0;

            // if a parameter update comes from GUI, we don't want to send an old (-ish) version of the same parameter to the GUI
            if gui_updates.contains(param_id) {
                continue;
            }
            // now we know we REALLY want to send this parameter update to the GUI
            unsafe {
                let update = ParameterUpdate {
                    parameter_id: param_id.clone(),
                    value: get_normalized_param_value(param_id.to_string(), &param_map),
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
    let normalized = update.value;
    let id = update.parameter_id.as_str();
    let param_ptr = get_param_ptr(id.to_owned(), map);

    setter.raw_context.raw_begin_set_parameter(param_ptr);
    setter
        .raw_context
        .raw_set_parameter_normalized(param_ptr, normalized);
    setter.raw_context.raw_end_set_parameter(param_ptr);
}

// TODO: is it even worth putting this in a function
unsafe fn get_normalized_param_value(id: String, map: &ParamMap) -> f32 {
    let param_ptr = get_param_ptr(id, map);
    param_ptr.modulated_normalized_value()
}

/// Get a `ParamPtr` given a parameter id and a param map.
fn get_param_ptr(id: String, map: &ParamMap) -> ParamPtr {
    map.iter()
        .find(|(param_id, _, _)| id == *param_id)
        .unwrap_or_else(|| panic!("Couldn't find a parameter with ID {}", id))
        .1
}
