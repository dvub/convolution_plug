use fundsp::hacker::AudioUnit;
use rubato::Resampler;

use crate::{
    config::{IrConfig, DEFAULT_NORMALIZATION_LEVEL},
    dsp::{convolve::convolver, resample::init_resampler},
    util::rms_normalize,
};

pub fn process_ir(
    ir_samples: &mut Vec<Vec<f32>>,
    ir_sample_rate: f32,
    sample_rate: f32,
    config: &IrConfig,
) -> anyhow::Result<()> {
    if config.resample && sample_rate > ir_sample_rate {
        let mut resampler = init_resampler(
            // TODO: problem?
            ir_samples.len(),
            ir_samples[0].len(),
            ir_sample_rate as f64,
            sample_rate as f64,
        )?;
        *ir_samples = resampler.process(&ir_samples, None)?
    };

    if config.normalize {
        rms_normalize(ir_samples, DEFAULT_NORMALIZATION_LEVEL);
    }

    Ok(())
}

// TODO: this might not be the best place for this function
pub fn init_convolvers(ir_samples: &Vec<Vec<f32>>) -> anyhow::Result<Box<dyn AudioUnit>> {
    // TODO: probably refactor this
    Ok(match ir_samples.as_slice() {
        [mono] => Box::new(convolver(mono) | convolver(mono)),
        [left, right] => Box::new(convolver(left) | convolver(right)),
        _ => todo!(),
    })
}
