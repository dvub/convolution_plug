use convolution::{fft_convolver::FFTConvolver, Convolution};

use fundsp::{hacker32::*, numeric_array::generic_array::GenericArray};

use std::sync::Arc;

use crate::ConvolutionPlugParams;

#[derive(Clone)]
pub struct ConvolverNode {
    convolver: FFTConvolver,
}

impl AudioNode for ConvolverNode {
    // TODO: fix this
    const ID: u64 = 0;

    type Inputs = U1;
    type Outputs = U1;

    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let mut output = [0.0];
        self.convolver.process(input, &mut output);
        Frame::new(GenericArray::from(output))
    }
}
pub fn convolver(samples: &[f32]) -> An<ConvolverNode> {
    let convolver =
        convolution::fft_convolver::FFTConvolver::init(samples, MAX_BUFFER_SIZE, samples.len());

    An(ConvolverNode { convolver })
}

/*
#[derive(Clone)]
pub struct ParamNode<F, N: Size<f32>> {
    _marker: PhantomData<N>,
    params: Arc<ConvolutionPlugParams>,
    accessor: F,
}

impl<F: Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync, N: Size<f32>> AudioNode
    for ParamNode<F, N>
{
    fn tick(&mut self, _: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        // let value = (self.accessor)(&self.params);
        // let a = vec![0.0; N::USIZE];
        Frame::generate(|_| 0.0)
    }
    // TODO: fix
    const ID: u64 = 0;

    type Inputs = U0;
    type Outputs = N;
}


// opcode (for convenience) (maybe not necessary)
pub fn gain<N: Size<f32>>(
    params: &Arc<ConvolutionPlugParams>,
) -> An<ParamNode<impl Fn(&ConvolutionPlugParams) -> f32 + Clone + Send + Sync, N>> {
    An(ParamNode::<_, N> {
        params: params.clone(),
        accessor: |params: &ConvolutionPlugParams| params.gain.value(),
        _marker: PhantomData,
    })
}
*/

pub fn dry_wet<N: Size<f32>>(params: &Arc<ConvolutionPlugParams>) -> Net {
    let clone = params.clone();

    Net::wrap(Box::new(
        envelope(move |_| clone.dry_wet.value()) >> split::<N>(),
    ))
}

pub fn gain<N: Size<f32>>(params: &Arc<ConvolutionPlugParams>) -> Net {
    let clone = params.clone();

    Net::wrap(Box::new(
        envelope(move |_| clone.gain.value()) >> split::<N>(),
    ))
}

pub fn lowpass_cutoff<N: Size<f32>>(params: &Arc<ConvolutionPlugParams>) -> Net {
    let clone = params.clone();

    Net::wrap(Box::new(
        envelope(move |_| clone.lowpass_cutoff.value()) >> split::<N>(),
    ))
}
pub fn lowpass_q<N: Size<f32>>(params: &Arc<ConvolutionPlugParams>) -> Net {
    let clone = params.clone();

    Net::wrap(Box::new(
        envelope(move |_| clone.lowpass_q.value()) >> split::<N>(),
    ))
}
