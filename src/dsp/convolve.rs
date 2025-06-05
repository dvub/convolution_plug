use convolution::{
    crossfade_convolver::CrossfadeConvolver, fft_convolver::FFTConvolver, Convolution,
};

use fundsp::hacker32::*;

/// This node is a light wrapper over the [fft-convolution](https://github.com/holoplot/fft-convolution) crate.
#[derive(Clone)]
pub struct ConvolverNode {
    convolver: CrossfadeConvolver<FFTConvolver>,
}

impl ConvolverNode {
    pub fn update(&mut self, new_response: &[f32]) {
        self.convolver.update(new_response);
    }
}

// TODO:
// implement other features from convolver such as crossfading etc
impl AudioNode for ConvolverNode {
    // TODO: fix this
    const ID: u64 = 0;

    type Inputs = U1;
    type Outputs = U1;

    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let mut output = [0.0];
        self.convolver.process(input, &mut output);
        Frame::from(output)
    }
}

// opcode
pub fn convolver(samples: &[f32]) -> An<ConvolverNode> {
    let convolver = FFTConvolver::init(samples, MAX_BUFFER_SIZE, samples.len());

    An(ConvolverNode {
        // TODO:
        // choose correct buffer length
        // choose correct crossfade_samples
        convolver: CrossfadeConvolver::new(convolver, samples.len(), MAX_BUFFER_SIZE, 1000),
    })
}
