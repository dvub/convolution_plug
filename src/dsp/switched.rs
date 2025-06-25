use std::{marker::PhantomData, ops::Sub};

use fundsp::{
    hacker32::*,
    typenum::{Sub1, B1},
};

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

    F: Fn(f32) -> bool + Send + Sync + Clone,
    <I as Sub<B1>>::Output: Send + Sync + Clone + Size<f32>,
{
    const ID: u64 = 0;

    type Inputs = I;
    type Outputs = Sub1<I>;

    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let mut output = Frame::default();

        let control_channel = I::USIZE - 1;
        let control_value = input[control_channel];

        if (self.control_fn)(control_value) {
            self.node.tick(input, &mut output);
        } else {
            /*
            let out = &input.clone()[0..control_channel];
            output = Frame::new(GenericArray::from_slice(out).clone());
             */
        }

        output
    }
}

impl<I, F> SwitchedNode<I, F>
where
    I: Size<f32> + Sub<B1>,
    F: Fn(f32) -> bool + Send + Sync + Clone,
{
    pub fn new(
        node: An<impl AudioNode<Inputs = Sub1<I>, Outputs = Sub1<I>> + 'static>,
        cfn: F,
    ) -> An<Self>
    where
        <I as Sub<B1>>::Output: Send + Sync + Clone + Size<f32>,
    {
        An(SwitchedNode {
            node: Box::new(node),
            control_fn: cfn,

            _marker: PhantomData,
        })
    }
}
