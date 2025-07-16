use super::ipc::{Message, ParameterUpdate};
use crate::{
    config::IRConfig,
    dsp::ir::init_convolvers,
    editor::ipc::{InitResponse, IrData},
    params::PluginParams,
    ConvolutionPlug,
};

type ParamMap = Vec<(String, ParamPtr, String)>;
const FADE_TIME: f64 = 1.0;
const FADE_TYPE: Fade = Fade::Smooth;

use fundsp::hacker32::*;
use itertools::Itertools;
use nih_plug::{
    params::Params,
    prelude::{ParamPtr, ParamSetter},
};
use nih_plug_webview::WindowHandler;
use serde_json::json;
use std::sync::{Arc, Mutex};

pub fn build_event_loop(
    plugin: &ConvolutionPlug,
) -> impl Fn(&WindowHandler, ParamSetter, &mut baseview::Window) + 'static + Send + Sync {
    let params = plugin.params.clone();
    let param_map = params.param_map();
    let param_update_rx = params.rx.clone();

    let sample_rate = plugin.sample_rate;
    let ir_slot = plugin.slot.clone();

    move |ctx: &WindowHandler, setter, _window| {
        // GUI -> BACKEND
        while let Ok(json_message) = ctx.next_event() {
            let message = serde_json::from_value::<Message>(json_message)
                .expect("Error reading message from GUI");
            match message {
                Message::Init => handle_init(ctx, &params),
                Message::ParameterUpdate(update) => unsafe {
                    handle_parameter_update(&update, &setter, &param_map);
                },
                // TODO: it's probably time for another refactor
                Message::IrUpdate(new_ir_data) => {
                    // TODO: fix unwraps?
                    let current_ir_config = params.ir_config.lock().unwrap().clone();
                    handle_ir_update(
                        &params,
                        &current_ir_config,
                        &ir_slot,
                        &new_ir_data,
                        sample_rate,
                    )
                    .unwrap();
                }
                // TODO: this is weirdly expensive - figure out why
                // if the decoding is the expensive part, we can store the samples somewhere and just repeatedly load them

                // TODO refactor
                Message::IrConfigUpdate(new_ir_config) => {
                    println!("RECEIVING NEW UPDATE: {:?}", new_ir_config);
                    // if an IR is already loaded, then we should reload it
                    // but if NO IR is loaded, that's pointless
                    if let Some(current_ir_data) = &*params.ir_data.lock().unwrap() {
                        // TODO: probably fix this unwrap
                        let convolvers =
                            init_convolvers(current_ir_data, sample_rate, &new_ir_config).unwrap();
                        ir_slot
                            .lock()
                            .unwrap()
                            .set(FADE_TYPE, FADE_TIME, convolvers);
                    }

                    println!("oke");
                    *params.ir_config.lock().unwrap() = new_ir_config;
                    println!("heya");
                }
                // we (the backend) should always be sending an init response, never receiving
                Message::InitResponse(..) => todo!(),
            }
        }
        // BACKEND -> GUI
        for param_index in param_update_rx.try_iter().unique() {
            unsafe {
                let message = Message::ParameterUpdate(ParameterUpdate::new(
                    param_index,
                    param_map[param_index].1.unmodulated_normalized_value(),
                ));
                ctx.send_json(json!(message));
            }
        }
    }
}

fn handle_init(ctx: &WindowHandler, params: &Arc<PluginParams>) {
    let param_map = params.param_map();

    let minimized_map: Vec<_> = param_map.iter().map(|(id, _, _)| id.clone()).collect();

    // TODO: is there something to be done about clone()?
    let config = params.ir_config.lock().unwrap().clone();

    let ir_data_lock = params.ir_data.lock().unwrap();
    // TODO: is this usage of unsafe correct?
    // should the whole function be unsafe?
    unsafe {
        let init_params: Vec<_> = param_map
            .iter()
            .enumerate()
            .map(|(i, (_, ptr, _))| ParameterUpdate::new(i, ptr.modulated_normalized_value()))
            .collect();

        let message = Message::InitResponse(InitResponse {
            param_map: minimized_map,
            init_params,
            ir_data: ir_data_lock.clone(),
            config,
        });
        ctx.send_json(json!(message));
    }
}

fn handle_ir_update(
    params: &Arc<PluginParams>,
    config: &IRConfig,
    slot: &Arc<Mutex<Slot>>,
    ir_data: &IrData,
    sample_rate: f32,
) -> anyhow::Result<()> {
    let convolvers = init_convolvers(ir_data, sample_rate, config)?;

    slot.lock().unwrap().set(FADE_TYPE, FADE_TIME, convolvers);

    *params.ir_data.lock().unwrap() = Some(ir_data.clone());

    Ok(())
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
