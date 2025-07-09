use rubato::Resampler;

use crate::{
    config::PluginConfig,
    dsp::resample::init_resampler,
    editor::ipc::IrData,
    util::{decode_samples, rms_normalize},
};

pub fn load_ir(
    ir_data: &IrData,
    sample_rate: f32,
    config: &PluginConfig,
) -> anyhow::Result<Vec<f32>> {
    let (decoded_channels, ir_sample_rate) = decode_samples(&ir_data.raw_bytes)?;
    // right now, we only use one channel
    let ir_samples = &decoded_channels[0];

    // TODO: resample only if SRs are not the same
    // ALSO only resample based on if its specified in config

    let mut resampler = init_resampler(ir_samples, ir_sample_rate as f64, sample_rate as f64)?;
    let resampled_output = &mut resampler.process(&[ir_samples], None)?[0];

    if config.normalize_irs {
        rms_normalize(resampled_output, config.normalization_level);
    }
    Ok(resampled_output.clone())
}
