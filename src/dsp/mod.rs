pub mod convolve;
pub mod param_nodes;
pub mod switched;

use fundsp::hacker32::*;
use std::sync::Arc;

use param_nodes::*;

use crate::{
    config::PluginConfig,
    dsp::{convolve::convolver, switched::switched_node},
    params::PluginParams,
    util::{read_samples_from_file, rms_normalize},
};

// TODO: is it smart to have this function be here?
pub fn build_graph(
    params: &Arc<PluginParams>,
    config: &PluginConfig,
) -> (Box<dyn AudioUnit>, Slot) {
    // 1. determine if and from where we should load an IR
    let samples = params.persistent_ir_samples.lock().unwrap();
    let slot_element: Box<dyn AudioUnit> = match samples.as_deref() {
        // if an IR was previously loaded, we detect that here and use it again
        Some(samples) => Box::new(convolver(samples) | convolver(samples)),
        None => {
            // if no IR was previously loaded, *then* we check if we should load anything
            // based on config
            if !config.default_ir_path.is_empty() {
                let mut samples = read_samples_from_file(&config.default_ir_path);
                if config.normalize_irs {
                    rms_normalize(&mut samples, config.normalization_level);
                }

                Box::new(convolver(&samples) | convolver(&samples))
            } else {
                println!("Passing through");
                // no IR is loaded.
                // we don't even have to convolve by an empty IR, e.g. [1.0, 0.0, 0.0 ... ],
                // we can simply pass the signal straight through for the best performance
                Box::new(pass() | pass())
            }
        }
    };
    // we want to update the IR/convolver dynamically, so we put it in a Slot
    let convolver_slot = Slot::new(slot_element);
    let slot_frontend = convolver_slot.0;
    let slot_backend = convolver_slot.1;

    let convolver = unit::<U2, U2>(Box::new(slot_backend));
    let eq_wet =
        convolver >> switched_lowpass(params) >> switched_bell(params) >> switched_highpass(params);

    let wet = eq_wet * dry_wet(params);
    let dry = multipass::<U2>() * (1.0 - dry_wet(params));
    let mixed = wet & dry;

    let graph = mixed * gain(params);

    (Box::new(graph), slot_frontend)
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
