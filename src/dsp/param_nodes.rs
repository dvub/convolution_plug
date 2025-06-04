use fundsp::hacker32::*;

use std::{marker::PhantomData, sync::Arc};

use crate::params::PluginParams;

// TODO:
// figure out some way to make params generic

#[derive(Clone)]
pub struct ParamNode<F, N: Size<f32>> {
    _marker: PhantomData<N>,
    params: Arc<PluginParams>,
    accessor: F,
}

impl<F, N> ParamNode<F, N>
where
    F: Fn(&PluginParams) -> f32 + Clone + Send + Sync,
    N: Size<f32>,
{
    fn new(params: &Arc<PluginParams>, accessor: F) -> An<Self> {
        An(Self {
            _marker: PhantomData,
            params: params.clone(),
            accessor,
        })
    }
}

impl<F, N> AudioNode for ParamNode<F, N>
where
    F: Fn(&PluginParams) -> f32 + Clone + Send + Sync,
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

pub trait AccessorFn: Fn(&PluginParams) -> f32 + Clone + Send + Sync {}
impl<F> AccessorFn for F where F: Fn(&PluginParams) -> f32 + Clone + Send + Sync {}

pub fn dry_wet<N: Size<f32>>(params: &Arc<PluginParams>) -> An<ParamNode<impl AccessorFn, N>> {
    ParamNode::new(params, |p| p.dry_wet.value())
}

pub fn gain<N: Size<f32>>(params: &Arc<PluginParams>) -> An<ParamNode<impl AccessorFn, N>> {
    ParamNode::new(params, |p| p.gain.value())
}

// named lp (instead of lowpass) so as to not collide with fundsp
pub fn lp_cutoff<N: Size<f32>>(params: &Arc<PluginParams>) -> An<ParamNode<impl AccessorFn, N>> {
    ParamNode::new(params, |p| p.lowpass_cutoff.value())
}

pub fn lp_q<N: Size<f32>>(params: &Arc<PluginParams>) -> An<ParamNode<impl AccessorFn, N>> {
    ParamNode::new(params, |p| p.lowpass_q.value())
}

pub fn lp_enabled<N: Size<f32>>(params: &Arc<PluginParams>) -> An<ParamNode<impl AccessorFn, N>> {
    ParamNode::new(params, |p| p.lowpass_enabled.value() as i32 as f32)
}
