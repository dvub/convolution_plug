// opcodes for all params

use std::sync::Arc;

use fundsp::hacker32::*;

use crate::{
    dsp::param_node::{Accessor, ParamNode},
    params::PluginParams,
};

pub fn lp_freq<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.lowpass_freq.smoothed.next())
}

pub fn lp_q<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.lowpass_q.smoothed.next())
}
pub fn lp_enabled<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| f32::from(u8::from(p.lowpass_enabled.value())))
}

// highpass
pub fn hp_enabled<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| f32::from(u8::from(p.highpass_enabled.value())))
}

pub fn hp_freq<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.highpass_freq.smoothed.next())
}
pub fn hp_q<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.highpass_q.smoothed.next())
}

// bell
pub fn bell_enabled<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| f32::from(u8::from(p.bell_enabled.value())))
}

pub fn bell_freq<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_freq.smoothed.next())
}
pub fn bell_q<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_q.smoothed.next())
}

pub fn bell_gain<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.bell_gain.smoothed.next())
}

pub fn wet_gain<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.wet_gain.smoothed.next())
}

pub fn dry_gain<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.dry_gain.smoothed.next())
}
