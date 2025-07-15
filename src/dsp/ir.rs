use fundsp::hacker::AudioUnit;
use rubato::Resampler;

use crate::{
    config::IRConfig,
    dsp::{convolve::convolver, resample::init_resampler},
    editor::ipc::IrData,
    util::{decode_samples, rms_normalize},
};

// TODO: maybe use a more generic return type?
fn init_ir(ir_data: &IrData, sample_rate: f32, config: &IRConfig) -> anyhow::Result<Vec<Vec<f32>>> {
    let (decoded_channels, ir_sample_rate) = decode_samples(&ir_data.raw_bytes)?;

    let mut output = if config.resample && sample_rate > ir_sample_rate {
        let mut resampler = init_resampler(
            // TODO: problem?
            decoded_channels.len(),
            decoded_channels[0].len(),
            ir_sample_rate as f64,
            sample_rate as f64,
        )?;

        resampler.process(&decoded_channels, None)?
    } else {
        decoded_channels
    };

    if config.normalize_irs {
        // TODO: make this function support stereo without this bullshit
        rms_normalize(&mut output, config.normalization_level);
    }

    Ok(output)
}

// TODO: this might not be the best place for this function
pub fn init_convolvers(
    ir_data: &IrData,
    sample_rate: f32,
    config: &IRConfig,
) -> anyhow::Result<Box<dyn AudioUnit>> {
    let ir_samples = init_ir(ir_data, sample_rate, config)?;

    // TODO: probably refactor this
    Ok(match ir_samples.as_slice() {
        [mono] => Box::new(convolver(mono) | convolver(mono)),
        [left, right] => Box::new(convolver(left) | convolver(right)),
        _ => todo!(),
    })
}
