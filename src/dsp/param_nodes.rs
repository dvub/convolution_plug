use fundsp::hacker32::*;

use std::{marker::PhantomData, sync::Arc};

use crate::params::ConvolutionPlugParams;

#[derive(Clone)]
pub struct ParamNode<F, N: Size<f32>> {
    _marker: PhantomData<N>,
    params: Arc<ConvolutionPlugParams>,
    accessor: F,
}

impl<F, N> ParamNode<F, N>
where
    F: Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync,
    N: Size<f32>,
{
    fn new(params: &Arc<ConvolutionPlugParams>, accessor: F) -> An<Self> {
        An(Self {
            _marker: PhantomData,
            params: params.clone(),
            accessor,
        })
    }
}

impl<F, N> AudioNode for ParamNode<F, N>
where
    F: Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync,
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

pub fn dry_wet<N: Size<f32>>(
    params: &Arc<ConvolutionPlugParams>,
) -> An<ParamNode<impl Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync, N>> {
    ParamNode::new(params, |p| p.dry_wet.value())
}

pub fn gain<N: Size<f32>>(
    params: &Arc<ConvolutionPlugParams>,
) -> An<ParamNode<impl Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync, N>> {
    ParamNode::new(params, |p| p.gain.value())
}

// named lp (instead of lowpass) so as to not collide with fundsp

pub fn lp_cutoff<N: Size<f32>>(
    params: &Arc<ConvolutionPlugParams>,
) -> An<ParamNode<impl Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync, N>> {
    ParamNode::new(params, |p| p.lowpass_cutoff.value())
}

pub fn lp_q<N: Size<f32>>(
    params: &Arc<ConvolutionPlugParams>,
) -> An<ParamNode<impl Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync, N>> {
    ParamNode::new(params, |p| p.lowpass_q.value())
}
