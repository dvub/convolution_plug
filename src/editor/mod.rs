mod ipc;

use std::{path::PathBuf, sync::Arc};

use nih_plug::{editor::Editor, params::Params};
use nih_plug_webview::{
    Context, EditorHandler, WebViewConfig, WebViewEditor, WebViewSource, WebViewState,
};
use serde_json::json;

use crate::{editor::ipc::Message, params::PluginParams};

pub struct PluginGui {
    params: Arc<PluginParams>,
}

impl PluginGui {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(state: &Arc<WebViewState>, params: &Arc<PluginParams>) -> Option<Box<dyn Editor>> {
        let config = WebViewConfig {
            title: String::from("My Plugin"),
            source: WebViewSource::URL(String::from("http://localhost:3000")),
            workdir: PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/target/webview-workdir"
            )),
        };
        let editor = WebViewEditor::new(
            PluginGui {
                params: params.clone(),
            },
            state,
            config,
        );

        Some(Box::new(editor))
    }
}

impl EditorHandler for PluginGui {
    fn init(&mut self, cx: &mut Context) {
        println!("INIT CALLED");
        cx.send_message("This is an init message".to_string());
    }

    fn on_frame(&mut self, cx: &mut Context) {}

    fn on_message(&mut self, send: &dyn Fn(String), message: String) {
        let m = json!(Message::Init).to_string();

        let str = send("Hello".to_string());
    }
}

fn handle_init(cx: &mut Context, params: &Arc<PluginParams>) {
    let param_map = params.param_map();

    let map_copy: Vec<_> = param_map.iter().map(|(id, _, _)| id.clone()).collect();

    let config = params.ir_config.lock().unwrap().clone();

    // let ir_data_lock = params.ir_data.lock().unwrap();

    let init_params: Vec<_> = param_map
        .iter()
        .enumerate()
        .map(|(i, (_, ptr, _))| unsafe {
            ParameterUpdate {
                parameter_index: i,
                value: ptr.modulated_normalized_value(),
            }
        })
        .collect();

    let message = Message::InitResponse(InitResponse {
        param_map: map_copy,
        init_params,
        ir_data: ir_data_lock.clone(),
        config,
    });
    ctx.send_json(json!(message));
}
