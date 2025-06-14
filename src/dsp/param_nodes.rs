use fundsp::hacker32::*;
use nih_plug::params::Params;

use crate::params::PluginParams;
use std::{marker::PhantomData, sync::Arc};

pub trait Accessor<P>: Fn(&Arc<P>) -> f32 + Clone + Send + Sync {}
impl<F, P> Accessor<P> for F where F: Fn(&Arc<P>) -> f32 + Clone + Send + Sync {}

/// An instance of this node represents an `nih-plug` parameter.
/// Every `tick()`, this node will run the provided closure and output its return value.
/// - 0 inputs.
/// - `N` outputs (allows using params with `N` signals).
///
/// The provided closure (the accessor) should return a parameter's value directly, or the next value from a smoother.
pub struct ParamNode<P, F, N>
where
    P: Params,
    F: Accessor<P>,
    N: Size<f32>,
{
    _marker: PhantomData<N>,
    params: Arc<P>,
    accessor: F,
}

// manually implement CLone here because rust complains otherwise
impl<P, F, N> Clone for ParamNode<P, F, N>
where
    P: Params,
    F: Accessor<P>,
    N: Size<f32>,
{
    fn clone(&self) -> Self {
        Self {
            params: self.params.clone(),
            accessor: self.accessor.clone(),
            _marker: PhantomData,
        }
    }
}

impl<P, F, N> AudioNode for ParamNode<P, F, N>
where
    P: Params + Send + Sync,
    F: Accessor<P>,
    N: Size<f32>,
{
    fn tick(&mut self, _: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let value = (self.accessor)(&self.params);
        Frame::splat(value)
    }
    // TODO: fix
    const ID: u64 = 0;

    type Inputs = U0;
    type Outputs = N;

    // this does not process samples, so process() is not needed
}

impl<P, F, N> ParamNode<P, F, N>
where
    P: Params,
    F: Accessor<P>,
    N: Size<f32>,
{
    /// Create a new `ParamNode`.
    /// This can be wrapped by functions to return a given parameter with an opcode, similarly to FunDSP's API.
    pub fn new(params: &Arc<P>, accessor: F) -> An<Self> {
        An(Self {
            _marker: PhantomData,
            params: params.clone(),
            accessor,
        })
    }
}
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
