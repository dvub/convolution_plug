use rubato::Resampler;

use crate::{
    config::PluginConfig,
    dsp::resample::init_resampler,
    editor::ipc::IrData,
    util::{decode_samples, rms_normalize},
};

pub fn init_ir(
    ir_data: &IrData,
    sample_rate: f32,
    config: &PluginConfig,
) -> anyhow::Result<Vec<Vec<f32>>> {
    let (decoded_channels, ir_sample_rate) = decode_samples(&ir_data.raw_bytes)?;

    let mut out = if config.resample && sample_rate > ir_sample_rate {
        let mut resampler = init_resampler(
            // TODO: problem?
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
        rms_normalize(&mut out[0], config.normalization_level);
        rms_normalize(&mut out[1], config.normalization_level);
    }

    Ok(out)
}
