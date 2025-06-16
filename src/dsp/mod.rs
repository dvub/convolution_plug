pub mod convolve;
pub mod param_nodes;

use fundsp::hacker32::*;
use std::sync::Arc;

use param_nodes::*;

use crate::params::PluginParams;
// yep this is the big thing
pub fn build_graph(p: &Arc<PluginParams>) -> (Box<dyn AudioUnit>, Slot) {
    // TODO: rewrite the entire EQ section to be not bad
    let lp = (lp_enabled(p)
        * ((multipass::<U2>() | lp_freq::<U2>(p) | lp_q::<U2>(p)) >> (lowpass() | lowpass())))
        & ((1.0 - lp_enabled(p)) * multipass::<U2>());

    let hp = (hp_enabled(p)
        * ((multipass::<U2>() | hp_freq::<U2>(p) | hp_q::<U2>(p)) >> (highpass() | highpass())))
        & ((1.0 - hp_enabled(p)) * multipass::<U2>());

    let bl = (bell_enabled(p)
        * ((multipass::<U2>()
            | bell_freq::<U2>(p)
            | param_nodes::bell_q::<U2>(p)
            | bell_gain::<U2>(p))
            >> (bell() | bell())))
        & ((1.0 - bell_enabled(p)) * multipass::<U2>());

    // no IR is loaded by default.
    // we don't even have to convolve by an empty IR, e.g. [1.0, 0.0, 0.0 ... ],
    // we can simply pass the signal straight through for the best performance
    let convolver_slot = Slot::new(Box::new(multipass::<U2>()));

    let slot_front = convolver_slot.0;
    let slot_back = convolver_slot.1;

    let convolver = unit::<U2, U2>(Box::new(slot_back));

    let eq_wet = convolver >> lp >> bl >> hp;

    let wet = eq_wet * dry_wet(p);
    let dry = multipass::<U2>() * (1.0 - dry_wet(p));

    let mixed = wet & dry;

    let graph = mixed * gain(p);

    (Box::new(graph), slot_front)
}
