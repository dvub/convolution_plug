use super::ipc::{Message, ParameterUpdate};
use crate::{
    config::PluginConfig,
    dsp::ir::init_convolvers,
    editor::ipc::{IrData, KnobGesture},
    params::PluginParams,
    ConvolutionPlug,
};

type ParamMap = Vec<(String, ParamPtr, String)>;
const FADE_TIME: f64 = 1.0;
const FADE_TYPE: Fade = Fade::Smooth;

use crossbeam_channel::{Receiver, TryIter};
use fundsp::hacker32::*;
use itertools::{Itertools, Unique};
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

    let config = params.config.lock().unwrap().clone();

    let dragged_params = Mutex::new(vec![false; param_map.len()]);
    move |ctx: &WindowHandler, setter, _window| {
        let mut dragged_params = dragged_params.lock().unwrap();

        // GUI -> BACKEND
        while let Ok(json_message) = ctx.next_event() {
            let message = serde_json::from_value::<Message>(json_message)
                .expect("Error reading message from GUI");
            match message {
                Message::Init => handle_init(ctx, &params),
                Message::ParameterUpdate(update) => unsafe {
                    handle_parameter_update(&update, &setter, &param_map);
                },
                Message::IrUpdate(ir_data) => {
                    // TODO: fix unwrap
                    handle_ir_update(&params, &config, &ir_slot, &ir_data, sample_rate).unwrap()
                }
                Message::Resize { .. } => todo!(),
                Message::KnobGesture {
                    gesture,
                    parameter_id,
                } => {
                    let index = param_map
                        .iter()
                        .position(|(id, _, _)| *id == parameter_id)
                        .unwrap();
                    match gesture {
                        KnobGesture::StartDrag => dragged_params[index] = true,
                        KnobGesture::StopDrag => dragged_params[index] = false,
                    }
                }
            }
        }
        // BACKEND -> GUI

        for param_index in get_unique_messages(&param_update_rx) {
            let param_id = &param_map[param_index].0;

            if dragged_params[param_index] {
                continue;
            }

            // println!("SENDING");

            // now we know we REALLY want to send this parameter update to the GUI
            // TODO: these string clones and whatnot might be expensive
            unsafe {
                let message = Message::ParameterUpdate(ParameterUpdate::new(
                    param_id.clone(),
                    get_normalized_param_value(param_id.clone(), &param_map),
                ));
                ctx.send_json(json!(message));
            }
        }
    }
}

fn handle_init(ctx: &WindowHandler, params: &Arc<PluginParams>) {
    let param_map = params.param_map();
    let ir_data_lock = params.ir_data.lock().unwrap();

    // TODO: figure out clone
    if let Some(ir_data) = ir_data_lock.as_ref() {
        ctx.send_json(json!(Message::IrUpdate(ir_data.clone())));
    }

    unsafe {
        for param_ptr in param_map {
            let message = Message::ParameterUpdate(ParameterUpdate::new(
                param_ptr.0.clone(),
                param_ptr.1.modulated_normalized_value(),
            ));
            ctx.send_json(json!(message));
        }
    }
}

fn handle_ir_update(
    params: &Arc<PluginParams>,
    config: &PluginConfig,
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

    // TODO: fix these fucking clone calls
    let param_id = &param_update.parameter_id;
    let param_ptr = get_param_ptr(param_id.clone(), param_map);

    param_setter.raw_context.raw_begin_set_parameter(param_ptr);
    param_setter
        .raw_context
        .raw_set_parameter_normalized(param_ptr, normalize_new_value);
    param_setter.raw_context.raw_end_set_parameter(param_ptr);
}

unsafe fn get_normalized_param_value(param_id: String, param_map: &ParamMap) -> f32 {
    let param_ptr = get_param_ptr(param_id, param_map);
    param_ptr.modulated_normalized_value()
}

/// Get a `ParamPtr` given a parameter id and a param map.
fn get_param_ptr(id: String, map: &ParamMap) -> ParamPtr {
    map.iter()
        .find(|(param_id, _, _)| id == *param_id)
        .unwrap_or_else(|| panic!("Couldn't find a parameter with ID {id}"))
        .1
}

fn get_unique_messages<T>(recv: &Receiver<T>) -> Unique<TryIter<'_, T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    recv.try_iter().unique()
}

#[cfg(test)]
mod tests {
    use crate::editor::event_loop::get_unique_messages;

    #[test]
    fn make_update_unique() -> anyhow::Result<()> {
        let (tx, rx) = crossbeam_channel::unbounded::<usize>();

        tx.send(1)?;
        tx.send(2)?;
        tx.send(1)?;

        let res: Vec<usize> = get_unique_messages(&rx).collect();

        assert_eq!(res, [1, 2]);

        Ok(())
    }
}
