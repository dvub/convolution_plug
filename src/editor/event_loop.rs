use super::ipc::{Message, ParameterUpdate};
use crate::{editor::ipc::InitResponse, params::PluginParams, ConvolutionPlug, Task};

type ParamMap = Vec<(String, ParamPtr, String)>;
pub const FADE_TIME: f64 = 1.0;
pub const FADE_TYPE: Fade = Fade::Smooth;

use fundsp::hacker32::*;
use itertools::Itertools;
use nih_plug::{
    params::Params,
    prelude::{AsyncExecutor, ParamPtr, ParamSetter},
};
use nih_plug_webview::WindowHandler;
use serde_json::json;
use std::sync::Arc;

pub fn build_event_loop(
    plugin: &ConvolutionPlug,
    async_executor: AsyncExecutor<ConvolutionPlug>,
) -> impl Fn(&WindowHandler, ParamSetter, &mut baseview::Window) + 'static + Send + Sync {
    let params = plugin.params.clone();
    let param_map = params.param_map();
    let param_update_rx = params.rx.clone();

    move |ctx: &WindowHandler, setter, _window| {
        // GUI -> BACKEND
        while let Ok(json_message) = ctx.next_event() {
            let message = serde_json::from_value::<Message>(json_message)
                .expect("Error reading message from GUI");
            match message {
                // right now i don't think these should be handled by the task executor
                Message::Init => handle_init(ctx, &params),
                Message::ParameterUpdate(update) => unsafe {
                    handle_parameter_update(&update, &setter, &param_map);
                },

                // these are more expensive, so we want to use the executor
                Message::IrUpdate(ir_data) => async_executor.execute_gui(Task::UpdateIr(ir_data)),
                Message::IrConfigUpdate(ir_config) => {
                    async_executor.execute_gui(Task::UpdateIrConfig(ir_config));
                }

                // we (the backend) should always be sending an init response, never receiving
                Message::InitResponse(..) => println!(
                    "WARNING: received an InitResponse on the GUI thread. This will be discarded. "
                ),
            }
        }

        // BACKEND -> GUI
        for param_index in param_update_rx.try_iter().unique() {
            unsafe {
                let message = Message::ParameterUpdate(ParameterUpdate {
                    parameter_index: param_index,
                    value: param_map[param_index].1.unmodulated_normalized_value(),
                });
                ctx.send_json(json!(message));
            }
        }
    }
}

fn handle_init(ctx: &WindowHandler, params: &Arc<PluginParams>) {
    let param_map = params.param_map();

    let map_copy: Vec<_> = param_map.iter().map(|(id, _, _)| id.clone()).collect();

    let config = params.ir_config.lock().unwrap().clone();

    let ir_data_lock = params.ir_data.lock().unwrap();
    // TODO: is this usage of unsafe correct?
    // should the whole function be unsafe?
    unsafe {
        let init_params: Vec<_> = param_map
            .iter()
            .enumerate()
            .map(|(i, (_, ptr, _))| ParameterUpdate {
                parameter_index: i,
                value: ptr.modulated_normalized_value(),
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
}

unsafe fn handle_parameter_update(
    param_update: &ParameterUpdate,
    param_setter: &ParamSetter,
    param_map: &ParamMap,
) {
    let normalize_new_value = param_update.value;

    let idx = param_update.parameter_index;
    let param_ptr = param_map[idx].1;

    param_setter.raw_context.raw_begin_set_parameter(param_ptr);
    param_setter
        .raw_context
        .raw_set_parameter_normalized(param_ptr, normalize_new_value);
    param_setter.raw_context.raw_end_set_parameter(param_ptr);
}
