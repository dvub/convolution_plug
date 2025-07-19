use fundsp::hacker::AudioUnit;
use rubato::Resampler;

use crate::{
    config::{IrConfig, DEFAULT_NORMALIZATION_LEVEL},
    dsp::{convolve::convolver, resample::init_resampler},
    util::rms_normalize,
};

pub fn process_ir(
    ir_samples: &[Vec<f32>],
    ir_sample_rate: f32,
    sample_rate: f32,
    config: &IrConfig,
) -> anyhow::Result<Vec<Vec<f32>>> {
    let mut out = if config.resample && sample_rate > ir_sample_rate {
        let mut resampler = init_resampler(
            // TODO: problem?
            ir_samples.len(),
            ir_samples[0].len(),
            ir_sample_rate as f64,
            sample_rate as f64,
        )?;
        resampler.process(ir_samples, None)?
    } else {
        // TODO: to_vec
        ir_samples.to_vec()
    };

    if config.normalize {
        rms_normalize(&mut out, DEFAULT_NORMALIZATION_LEVEL);
    }

    Ok(out)
}

pub fn init_convolvers(ir_samples: &[Vec<f32>]) -> Box<dyn AudioUnit> {
    // TODO: is there a better way?
    match ir_samples {
        [mono] => Box::new(convolver(mono) | convolver(mono)),
        [left, right] => Box::new(convolver(left) | convolver(right)),
        _ => todo!(),
    }
}
