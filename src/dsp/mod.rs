pub mod convolve;
pub mod nodes;
pub mod param_node;
pub mod param_node_shared;
pub mod switched;

use fundsp::hacker32::*;
use std::sync::Arc;

use nodes::*;

use crate::{dsp::switched::switched_node, params::PluginParams};

// yep this is the big thing
pub fn build_graph(p: &Arc<PluginParams>) -> (Box<dyn AudioUnit>, Slot) {
    // no IR is loaded by default.
    // we don't even have to convolve by an empty IR, e.g. [1.0, 0.0, 0.0 ... ],
    // we can simply pass the signal straight through for the best performance
    let convolver_slot = Slot::new(Box::new(multipass::<U2>()));

    let slot_front = convolver_slot.0;
    let slot_back = convolver_slot.1;

    let convolver = unit::<U2, U2>(Box::new(slot_back));
    let eq_wet = convolver >> switched_lowpass(p) >> switched_bell(p) >> switched_highpass(p);

    let wet = eq_wet * dry_wet(p);
    let dry = multipass::<U2>() * (1.0 - dry_wet(p));
    let mixed = wet & dry;

    let graph = mixed * gain(p);

    (Box::new(graph), slot_front)
}

fn lp_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | lp_freq::<U1>(p) | lp_q::<U1>(p)) >> lowpass()
}

fn hp_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | hp_freq::<U1>(p) | hp_q::<U1>(p)) >> highpass()
}

fn bell_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | bell_freq::<U1>(p) | nodes::bell_q::<U1>(p) | bell_gain::<U1>(p)) >> bell()
}

// ...

fn switched_bell(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    (multipass::<U2>() | bell_enabled::<U1>(p))
        >> switched_node(stacki::<U2, _, _>(|_| bell_with_params(p)), |x| x == 1.0)
}

fn switched_lowpass(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    (multipass::<U2>() | lp_enabled::<U1>(p))
        >> switched_node(stacki::<U2, _, _>(|_| lp_with_params(p)), |x| x == 1.0)
}

fn switched_highpass(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    (multipass::<U2>() | hp_enabled::<U1>(p))
        >> switched_node(stacki::<U2, _, _>(|_| hp_with_params(p)), |x| x == 1.0)
}
