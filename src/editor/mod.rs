mod embedded;
pub mod ipc;

use std::{path::PathBuf, sync::Arc};

use nih_plug::{editor::Editor, params::Params, prelude::AsyncExecutor};
use nih_plug_webview::{
    Context, EditorHandler, WebViewConfig, WebViewEditor, WebViewSource, WebViewState,
};
use serde_json::json;

#[cfg(not(debug_assertions))]
use crate::editor::embedded::embedded_editor;
use crate::{
    editor::ipc::{InitResponse, Message, ParameterUpdate},
    params::PluginParams,
    ConvolutionPlug, Task,
};

pub struct PluginGui {
    params: Arc<PluginParams>,
    executor: AsyncExecutor<ConvolutionPlug>,
}

impl PluginGui {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        state: &Arc<WebViewState>,
        params: &Arc<PluginParams>,
        exec: AsyncExecutor<ConvolutionPlug>,
    ) -> Option<Box<dyn Editor>> {
        #[cfg(debug_assertions)]
        let editor = dev_editor(state, params, exec);

        #[cfg(not(debug_assertions))]
        let editor = embedded_editor(state, params, exec);

        Some(Box::new(editor))
    }
}

fn dev_editor(
    state: &Arc<WebViewState>,
    params: &Arc<PluginParams>,
    exec: AsyncExecutor<ConvolutionPlug>,
) -> WebViewEditor {
    let config = WebViewConfig {
        title: "Convolution".to_string(),
        source: WebViewSource::URL(String::from("http://localhost:3000")),
        workdir: PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/webview-workdir"
        )),
    };

    WebViewEditor::new(
        PluginGui {
            params: params.clone(),
            executor: exec,
        },
        state,
        config,
    )
}

impl EditorHandler for PluginGui {
    fn on_frame(&mut self, cx: &mut Context) {
        if cx.params_changed() {
            let updates = build_param_update_array(&self.params);
            let message = Message::ParameterUpdate(updates);

            cx.send_message(json!(message).to_string());
        }
    }

    fn on_message(&mut self, cx: &mut Context, message: String) {
        // TODO: is it correct to expect()
        let message =
            serde_json::from_str::<Message>(&message).expect("Error reading message from GUI");

        match message {
            Message::Init => self.handle_init(cx),
            Message::ParameterUpdate(parameter_updates) => {
                self.handle_parameter_update(cx, &parameter_updates)
            }
            // we gotta use task executor for this..
            Message::IrUpdate(ir_data) => self.executor.execute_gui(Task::UpdateIr(ir_data)),
            Message::IrConfigUpdate(ir_config) => {
                self.executor.execute_gui(Task::UpdateIrConfig(ir_config));
            }

            Message::InitResponse(..) => println!(
                "WARNING: received an InitResponse on the GUI thread. This will be discarded. "
            ),
            Message::Resize { width, height } => {
                cx.resize_window(width, height);
            }
        }
    }
}
impl PluginGui {
    fn handle_init(&self, cx: &mut Context) {
        let params = &self.params;

        let config = params.ir_config.lock().unwrap().clone();
        let ir_data_lock = params.ir_data.lock().unwrap();
        let init_params = build_param_update_array(params);

        let message = Message::InitResponse(InitResponse {
            init_params,
            ir_data: ir_data_lock.clone(),
            config,
        });
        cx.send_message(json!(message).to_string());
    }

    fn handle_parameter_update(&self, cx: &mut Context, param_updates: &Vec<ParameterUpdate>) {
        let param_map = self.params.param_map();
        let param_setter = cx.get_setter();

        for param_update in param_updates {
            let normalize_new_value = param_update.value;

            let id = &param_update.parameter_id;
            let param_ptr = param_map
                .iter()
                .find(|(map_id, _, _)| map_id == id)
                .unwrap()
                .1;

            unsafe {
                param_setter.raw_context.raw_begin_set_parameter(param_ptr);
                param_setter
                    .raw_context
                    .raw_set_parameter_normalized(param_ptr, normalize_new_value);
                param_setter.raw_context.raw_end_set_parameter(param_ptr);
            }
        }
    }
}

fn build_param_update_array(params: &Arc<PluginParams>) -> Vec<ParameterUpdate> {
    let param_map = params.param_map();
    param_map
        // TODO: issue with into_iter?
        .into_iter()
        .map(|(id, ptr, _)| ParameterUpdate {
            parameter_id: id,
            // TODO: double-check that this usage of unsafe is appropriate
            value: unsafe { ptr.unmodulated_normalized_value() },
        })
        .collect()
}
