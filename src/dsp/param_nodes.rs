// opcodes for all params.

use std::sync::Arc;

use fundsp::hacker32::*;

use crate::params::PluginParams;
use np_fundsp_bridge::params::{Accessor, ParamNode};

// NOTE:
// if we want to do param smoothing stuff, its easy to implement in the accessor function

pub fn gain<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.gain.value())
}
pub fn dry_wet<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.dry_wet.value())
}

// wet EQ params

pub fn lp_freq<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.lowpass_freq.value())
}

pub fn lp_q<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.lowpass_q.value())
}
pub fn lp_enabled<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.lowpass_enabled.value() as i32 as f32)
}

// highpass
pub fn hp_enabled<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.highpass_enabled.value() as i32 as f32)
}

pub fn hp_freq<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.highpass_freq.value())
}
pub fn hp_q<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.highpass_q.value())
}

// bell
pub fn bell_enabled<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_enabled.value() as i32 as f32)
}

pub fn bell_freq<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_freq.value())
}
pub fn bell_q<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_q.value())
}

pub fn bell_gain<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_gain.value())
}
