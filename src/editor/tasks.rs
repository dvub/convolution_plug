use std::sync::{Arc, Mutex};

use fundsp::slot::Slot;
use nih_plug::plugin::TaskExecutor;

use crate::{
    dsp::{convolve::init_convolvers, FADE_TIME, FADE_TYPE},
    editor::ipc::IrData,
    params::PluginParams,
    processing::{config::IrProcessingConfig, decode::decode_samples, process_ir},
    ConvolutionPlug,
};

pub enum Task {
    UpdateIrConfig(IrProcessingConfig),
    UpdateIr(IrData),
}

// TODO: rename this function?
pub fn handle_task(plugin: &mut ConvolutionPlug) -> TaskExecutor<ConvolutionPlug> {
    let params = plugin.params.clone();
    let slot = plugin.slot.clone();
    let sample_rate = plugin.sample_rate;

    Box::new(move |task| match task {
        Task::UpdateIrConfig(new_ir_config) => {
            update_ir_config(new_ir_config, &params, &slot, sample_rate)
        }
        Task::UpdateIr(ir_data) => update_ir(ir_data, &params, &slot, sample_rate),
    })
}
// TODO: these functions share some logic that could probably be refactored
fn update_ir(
    ir_data: IrData,
    params: &Arc<PluginParams>,
    slot: &Arc<Mutex<Slot>>,
    sample_rate: f32,
) {
    let config = params.ir_config.lock().unwrap();

    let (plain_ir_samples, ir_sample_rate) =
        decode_samples(&ir_data.raw_bytes).expect("There was an error decoding the file");

    *params.ir_samples.lock().unwrap() = (plain_ir_samples.clone(), ir_sample_rate);

    let processed_ir = process_ir(&plain_ir_samples, ir_sample_rate, sample_rate, &config)
        .expect("There was an error processing this IR");

    let convolvers = init_convolvers(&processed_ir);

    slot.lock().unwrap().set(FADE_TYPE, FADE_TIME, convolvers);
    *params.ir_data.lock().unwrap() = Some(ir_data);
}

fn update_ir_config(
    new_ir_config: IrProcessingConfig,
    params: &Arc<PluginParams>,
    slot: &Arc<Mutex<Slot>>,
    sample_rate: f32,
) {
    let (ir_samples, ir_sample_rate) = &*params.ir_samples.lock().unwrap();

    // if IR is already loaded
    if !ir_samples.is_empty() {
        let processed_ir = process_ir(ir_samples, *ir_sample_rate, sample_rate, &new_ir_config)
            .expect("There was an error processing this IR");

        let convolvers = init_convolvers(&processed_ir);

        slot.lock().unwrap().set(FADE_TYPE, FADE_TIME, convolvers);
    }

    // regardless of if IR is loaded, make sure the new config is persistent
    *params.ir_config.lock().unwrap() = new_ir_config;
}
