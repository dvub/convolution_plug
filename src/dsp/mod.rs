pub mod convolve;
pub mod param_nodes;

use fundsp::hacker32::*;
use std::sync::Arc;

use {convolve::convolver, param_nodes::*};

use crate::params::PluginParams;
// yep this is the big thing
pub fn build_graph(p: &Arc<PluginParams>, ir_samples: &[f32]) -> (Box<dyn AudioUnit>, Slot) {
    // TODO: rewrite the entire EQ section to be not bad

    let lp = (lp_enabled(p) * ((pass() | lp_freq::<U1>(p) | lp_q::<U1>(p)) >> lowpass()))
        & ((1.0 - lp_enabled(p)) * pass());

    let hp = (hp_enabled(p) * ((pass() | hp_freq::<U1>(p) | hp_q::<U1>(p)) >> highpass()))
        & ((1.0 - hp_enabled(p)) * pass());

    let bl = (bell_enabled(p)
        * ((pass() | bell_freq::<U1>(p) | param_nodes::bell_q::<U1>(p) | bell_gain::<U1>(p))
            >> bell()))
        & ((1.0 - bell_enabled(p)) * pass());

    let convolver_slot = Slot::new(Box::new(convolver(ir_samples)));

    let slot_front = convolver_slot.0;
    let slot_back = convolver_slot.1;

    let eq_wet = unit::<U1, U1>(Box::new(slot_back)) >> lp >> bl >> hp;

    let wet = eq_wet * dry_wet(p);
    let dry = pass() * (1.0 - dry_wet(p));

    let mixed = wet & dry;

    let graph = ((mixed >> split::<U2>()) * gain(p));

    (Box::new(graph), slot_front)
}
