use fundsp::hacker32::*;
use nih_plug::params::Params;

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

    // i have chosen this number completely randomly
    const ID: u64 = 1202;

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
    /// This can be wrapped by functions to return a given parameter with an opcode, similarly to `FunDSP`'s API.
    pub fn new(params: &Arc<P>, accessor: F) -> An<Self> {
        An(Self {
            _marker: PhantomData,
            params: params.clone(),
            accessor,
        })
    }
}
