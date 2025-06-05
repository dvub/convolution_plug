mod convolve;
pub mod param_nodes;

use fundsp::hacker32::*;
use std::{any::Any, sync::Arc};

use {
    convolve::convolver,
    param_nodes::{dry_wet, gain, lp_cutoff, lp_enabled, lp_q},
};

use crate::{
    params::PluginParams,
    util::{read_samples_from_file, rms_normalize},
};
// yep this is the big thing
pub fn build_graph(p: &Arc<PluginParams>) -> Box<dyn AudioUnit> {
    // let path = "D:\\projects\\rust\\convolution_plug\\test_irs\\large.wav";
    let path = "C:\\Users\\Kaya\\Documents\\projects\\convolution_plug\\test_irs\\large.wav";

    let mut ir_samples = read_samples_from_file(path);
    rms_normalize(&mut ir_samples, -48.0);

    let lp = (lp_enabled(p)
        * ((multipass::<U1>() | lp_cutoff::<U1>(p) | lp_q::<U1>(p)) >> lowpass()))
        & ((1.0 - lp_enabled(p)) * multipass::<U1>());

    let fuhhh = convolver(&ir_samples);
    let mono_wet = (fuhhh) >> lp;

    let wet = mono_wet * dry_wet(p);
    let dry = pass() * (1.0 - dry_wet(p));

    let mixed = wet & dry;

    let graph = (mixed >> split::<U2>()) * gain(p);

    Box::new(graph)
}
