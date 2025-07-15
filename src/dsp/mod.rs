pub mod convolve;
pub mod ir;
pub mod param_nodes;
pub mod switched;

mod params;

mod resample;

use fundsp::hacker32::*;
use std::sync::Arc;

use param_nodes::*;

use crate::{
    config::IRConfig,
    dsp::{ir::init_convolvers, switched::switched_node},
    params::PluginParams,
};

pub fn build_graph(
    params: &Arc<PluginParams>,
    config: &IRConfig,
    sample_rate: f32,
) -> anyhow::Result<(Box<dyn AudioUnit>, Slot)> {
    // 1. determine if and from where we should load an IR
    let mut ir_data = params.ir_data.lock().unwrap();
    let slot_element: Box<dyn AudioUnit> = match &mut *ir_data {
        // if an IR was previously loaded, we detect that here and use it again
        Some(ir_data) => init_convolvers(ir_data, sample_rate, config)?,
        None => Box::new(multipass::<U2>() * 0.0),
    };
    // we want to update the IR/convolver dynamically, so we put it in a Slot
    let convolver_slot = Slot::new(slot_element);
    let slot_frontend = convolver_slot.0;
    let slot_backend = convolver_slot.1;

    let convolver = unit::<U2, U2>(Box::new(slot_backend));
    let eq_wet =
        convolver >> switched_lowpass(params) >> switched_bell(params) >> switched_highpass(params);

    let wet = eq_wet * wet_gain(params);
    let dry = multipass::<U2>() * dry_gain(params);
    let graph = wet & dry;

    Ok((Box::new(graph), slot_frontend))
}

fn lp_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | lp_freq::<U1>(p) | lp_q::<U1>(p)) >> lowpass()
}

fn hp_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | hp_freq::<U1>(p) | hp_q::<U1>(p)) >> highpass()
}

fn bell_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | bell_freq::<U1>(p) | param_nodes::bell_q::<U1>(p) | bell_gain::<U1>(p)) >> bell()
}

// ...

fn switched_bell(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    let stereo_bell = bell_with_params(p) | bell_with_params(p);
    (multipass::<U2>() | bell_enabled::<U1>(p)) >> switched_node(stereo_bell, |x| x == 1.0)
}

fn switched_lowpass(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    let stereo_lowpass = lp_with_params(p) | lp_with_params(p);
    (multipass::<U2>() | lp_enabled::<U1>(p)) >> switched_node(stereo_lowpass, |x| x == 1.0)
}

fn switched_highpass(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    let stereo_highpass = hp_with_params(p) | hp_with_params(p);
    (multipass::<U2>() | hp_enabled::<U1>(p)) >> switched_node(stereo_highpass, |x| x == 1.0)
}
