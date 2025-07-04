use fundsp::{
    hacker32::*,
    typenum::{Sub1, B1},
};
use std::{marker::PhantomData, ops::Sub};

/// A node which can variably process its input given a condition.
/// The opcode itself takes in a `node` (or nodes), (`N` channels), and a closure.
/// The control value is passed into the closure, and if true, will process the input through `node`
/// - First N inputs: input signal
/// - Final input: control value (e.g. `var`, `dc`, etc.)
/// - N Outputs, which may be proccesed by `node`
#[derive(Clone)]
pub struct SwitchedNode<I, F>
where
    I: Size<f32>,
    F: Fn(f32) -> bool + Send + Sync,
{
    node: Box<dyn AudioUnit>,
    control_fn: F,
    _marker: PhantomData<I>,
}

impl<I, F> AudioNode for SwitchedNode<I, F>
where
    I: Size<f32> + Sub<B1>,
    <I as Sub<B1>>::Output: Send + Sync + Clone + Size<f32>,
    F: Fn(f32) -> bool + Send + Sync + Clone,
{
    const ID: u64 = 0;

    type Inputs = I;
    type Outputs = Sub1<I>;

    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        // TODO: is default incorrect?
        let mut output = Frame::default();

        // here we split apart the input
        let control_channel = I::USIZE - 1;
        let control_value = input[control_channel];
        let actual_input = &input[0..control_channel];

        // actually decide whether to do expensive computations or not
        if (self.control_fn)(control_value) {
            self.node.tick(actual_input, &mut output);
        } else {
            // TODO:
            // possibly optimize?
            output.copy_from_slice(actual_input);
        }

        output
    }
}

impl<I, F> SwitchedNode<I, F>
where
    I: Size<f32> + Sub<B1>,
    F: Fn(f32) -> bool + Send + Sync + Clone,
{
    fn new(
        node: An<impl AudioNode<Inputs = Sub1<I>, Outputs = Sub1<I>> + 'static>,
        control_fn: F,
    ) -> An<Self>
    where
        <I as Sub<B1>>::Output: Send + Sync + Clone + Size<f32>,
    {
        An(SwitchedNode {
            node: Box::new(node),
            control_fn,

            _marker: PhantomData,
        })
    }
}

// TODO:
// should new() return An<>, or this opcode?
pub fn switched_node<I, F>(
    node: An<impl AudioNode<Inputs = Sub1<I>, Outputs = Sub1<I>> + 'static>,
    control_fn: F,
) -> An<SwitchedNode<I, F>>
where
    I: Sub<B1> + Send + Sync + Clone + Size<f32>,
    <I as Sub<B1>>::Output: Send + Sync + Clone + Size<f32>,
    F: Fn(f32) -> bool + Send + Sync + Clone,
{
    SwitchedNode::new(node, control_fn)
}
