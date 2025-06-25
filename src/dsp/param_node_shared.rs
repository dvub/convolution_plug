use fundsp::hacker32::*;
use nih_plug::params::Params;

use std::sync::{atomic::AtomicU32, Arc};

pub trait Accessor<P>: Fn(&Arc<P>) -> f32 + Clone + Send + Sync {}
impl<F, P> Accessor<P> for F where F: Fn(&Arc<P>) -> f32 + Clone + Send + Sync {}

/// An instance of this node represents an `nih-plug` parameter.
/// Every `tick()`, this node will run the provided closure and output its return value.
/// - 0 inputs.
/// - `N` outputs (allows using params with `N` signals).
///
/// The provided closure (the accessor) should return a parameter's value directly, or the next value from a smoother.
pub struct ParamNodeShared<P, F>
where
    P: Params,
    F: Accessor<P>,
{
    params: Arc<P>,
    accessor: F,
    shared: Arc<AtomicU32>,
}

// manually implement CLone here because rust complains otherwise
impl<P, F> Clone for ParamNodeShared<P, F>
where
    P: Params,
    F: Accessor<P>,
{
    fn clone(&self) -> Self {
        Self {
            params: self.params.clone(),
            accessor: self.accessor.clone(),
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<P, F> AudioNode for ParamNodeShared<P, F>
where
    P: Params + Send + Sync,
    F: Accessor<P>,
{
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let value = (self.accessor)(&self.params);
        f32::store(&self.shared, value);

        *input
    }
    // TODO: fix
    const ID: u64 = 0;

    type Inputs = U1;
    type Outputs = U1;

    // this does not process samples, so process() is not needed
}

impl<P, F> ParamNodeShared<P, F>
where
    P: Params,
    F: Accessor<P>,
{
    /// Create a new `ParamNode`.
    /// This can be wrapped by functions to return a given parameter with an opcode, similarly to FunDSP's API.
    pub fn new(params: &Arc<P>, accessor: F, shared: &Shared) -> An<Self> {
        An(Self {
            params: params.clone(),
            shared: Arc::clone(shared.get_shared()),
            accessor,
        })
    }
}
