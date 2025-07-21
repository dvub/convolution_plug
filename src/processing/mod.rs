pub mod config;
pub mod decode;
pub mod normalize;
pub mod resample;

use config::{IrProcessingConfig, DEFAULT_NORMALIZATION_LEVEL};
use normalize::rms_normalize;
use resample::init_resampler;

use rubato::Resampler;

pub fn process_ir(
    ir_samples: &[Vec<f32>],
    ir_sample_rate: f32,
    sample_rate: f32,
    config: &IrProcessingConfig,
) -> anyhow::Result<Vec<Vec<f32>>> {
    let mut out = if config.resample && sample_rate > ir_sample_rate {
        let mut resampler = init_resampler(
            ir_samples.len(),
            // NOTE!!
            // would this be a problem is channels are (somehow) different sizes?
            ir_samples[0].len(),
            ir_sample_rate.into(),
            sample_rate.into(),
        )?;
        resampler.process(ir_samples, None)?
    } else {
        ir_samples.to_vec()
    };

    if config.normalize {
        rms_normalize(&mut out, DEFAULT_NORMALIZATION_LEVEL);
    }

    Ok(out)
}
