use convolution::{fft_convolver::FFTConvolver, Convolution};

use fundsp::{hacker32::*, numeric_array::generic_array::GenericArray};

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
