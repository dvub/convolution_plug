use fundsp::hacker32::*;
use nih_plug::params::Params;

use crate::params::PluginParams;
use std::{marker::PhantomData, sync::Arc};

pub trait Accessor<P>: Fn(&Arc<P>) -> f32 + Clone + Send + Sync {}
impl<F, P> Accessor<P> for F where F: Fn(&Arc<P>) -> f32 + Clone + Send + Sync {}

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

    // TODO: is process() necessary??
}

impl<P, F, N> ParamNode<P, F, N>
where
    P: Params,
    F: Accessor<P>,
    N: Size<f32>,
{
    fn new(params: &Arc<P>, accessor: F) -> An<Self> {
        An(Self {
            _marker: PhantomData,
            params: params.clone(),
            accessor,
        })
    }
}
// NOTE:
// if we want to do smoothing stuff, its easy to implement in the accessor function

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
pub fn lp_cutoff<N: Size<f32>>(
    p: &Arc<PluginParams>,
) -> An<ParamNode<PluginParams, impl Accessor<PluginParams>, N>> {
    ParamNode::new(p, |p| p.lowpass_cutoff.value())
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
