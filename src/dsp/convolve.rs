use convolution::{fft_convolver::FFTConvolver, Convolution};

use fundsp::hacker32::*;

/// This node is a light wrapper over the [fft-convolution](https://github.com/holoplot/fft-convolution) crate.
/// ### Note
/// Switching out an IR can be done with FunDSP's real-time features, such as swapping nodes within a `Net`.
#[derive(Clone)]
pub struct Convolver {
    convolver: FFTConvolver,
}

impl AudioNode for Convolver {
    const ID: u64 = 1203;

    type Inputs = U1;
    type Outputs = U1;
    /// performance is horrible, avoid calling tick() if possible
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let mut output = [0.0];
        self.convolver.process(input, &mut output);
        Frame::from(output)
    }

    // this allows block processing to occur, which is... better.
    fn process(&mut self, _size: usize, input: &BufferRef, output: &mut BufferMut) {
        self.convolver
            .process(input.channel_f32(0), output.channel_f32_mut(0));
    }
}

// seemed to perform the best?
const BLOCK_SIZE: usize = 1024;

// opcode
pub fn convolver(samples: &[f32]) -> An<Convolver> {
    let convolver = FFTConvolver::init(samples, BLOCK_SIZE, samples.len());

    An(Convolver { convolver })
}
