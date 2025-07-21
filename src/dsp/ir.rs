use fundsp::hacker::AudioUnit;

use crate::dsp::convolve::convolver;

// TODO: move this
pub fn init_convolvers(ir_samples: &[Vec<f32>]) -> Box<dyn AudioUnit> {
    // is there a better way?
    match ir_samples {
        [mono] => Box::new(convolver(mono) | convolver(mono)),
        [left, right] => Box::new(convolver(left) | convolver(right)),
        _ => todo!(),
    }
}
