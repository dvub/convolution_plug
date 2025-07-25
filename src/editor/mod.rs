pub mod ipc;

use std::{path::PathBuf, sync::Arc};

use itertools::Itertools;
use nih_plug::{editor::Editor, params::Params, prelude::AsyncExecutor};
use nih_plug_webview::{
    Context, EditorHandler, WebViewConfig, WebViewEditor, WebViewSource, WebViewState,
};
use serde_json::json;

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
                executor: exec,
            },
            state,
            config,
        );

        Some(Box::new(editor))
    }
}

impl EditorHandler for PluginGui {
    fn on_frame(&mut self, cx: &mut Context) {
        let param_map = self.params.param_map();
        let rx = &self.params.callback_handler.rx;

        for param_index in rx.try_iter().unique() {
            let message = Message::ParameterUpdate(ParameterUpdate {
                parameter_index: param_index,
                // TODO: double-check that this usage of unsafe is appropriate
                value: unsafe { param_map[param_index].1.unmodulated_normalized_value() },
            });
            cx.send_message(json!(message).to_string());
        }
    }

    fn on_message(&mut self, cx: &mut Context, message: String) {
        // TODO: is it correct to expect()
        let message = serde_json::from_slice::<Message>(message.as_bytes())
            .expect("Error reading message from GUI");

        match message {
            Message::Init => self.handle_init(cx),
            Message::ParameterUpdate(parameter_update) => {
                self.handle_parameter_update(cx, &parameter_update)
            }
            // we gotta use task executor for this..
            Message::IrUpdate(ir_data) => self.executor.execute_gui(Task::UpdateIr(ir_data)),
            Message::IrConfigUpdate(ir_config) => {
                self.executor.execute_gui(Task::UpdateIrConfig(ir_config));
            }

            Message::InitResponse(..) => println!(
                "WARNING: received an InitResponse on the GUI thread. This will be discarded. "
            ),
        }
    }
}
impl PluginGui {
    fn handle_init(&self, cx: &mut Context) {
        let params = &self.params;
        let param_map = params.param_map();

        let map_copy: Vec<_> = param_map.iter().map(|(id, _, _)| id.clone()).collect();

        let config = params.ir_config.lock().unwrap().clone();

        let ir_data_lock = params.ir_data.lock().unwrap();

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
        cx.send_message(json!(message).to_string());
    }

    fn handle_parameter_update(&self, cx: &mut Context, param_update: &ParameterUpdate) {
        let param_map = self.params.param_map();
        let param_setter = cx.get_setter();

        let normalize_new_value = param_update.value;
        let idx = param_update.parameter_index;
        let param_ptr = param_map[idx].1;

        unsafe {
            param_setter.raw_context.raw_begin_set_parameter(param_ptr);
            param_setter
                .raw_context
                .raw_set_parameter_normalized(param_ptr, normalize_new_value);
            param_setter.raw_context.raw_end_set_parameter(param_ptr);
        }
    }
}
