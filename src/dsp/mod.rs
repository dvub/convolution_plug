mod convolve;
pub mod param_nodes;

use fundsp::hacker32::*;
use std::sync::Arc;

use {
    convolve::convolver,
    param_nodes::{dry_wet, gain, lp_cutoff, lp_enabled, lp_q},
};

use crate::params::PluginParams;
// yep this is the big thing
pub fn build_graph(p: &Arc<PluginParams>, ir_samples: &[f32]) -> Box<dyn AudioUnit> {
    let lp = (lp_enabled(p)
        * ((multipass::<U1>() | lp_cutoff::<U1>(p) | lp_q::<U1>(p)) >> lowpass()))
        & ((1.0 - lp_enabled(p)) * multipass::<U1>());

    let mut convolver_slot = Slot::new(Box::new(convolver(ir_samples)));

    let slot_front = convolver_slot.0;
    let slot_back = Box::new(convolver_slot.1);
    let mut net = Net::new(1, 1);

    let mono_wet = slot_back >> lp;

    let wet = mono_wet * dry_wet(p);
    let dry = pass() * (1.0 - dry_wet(p));

    let mixed = wet & dry;

    let graph = (mixed >> split::<U2>()) * gain(p);

    Box::new(graph)
}
