use rubato::Resampler;

use crate::{
    config::PluginConfig,
    dsp::resample::init_resampler,
    editor::ipc::IrData,
    util::{decode_ir_samples, rms_normalize},
};

pub fn load_ir(ir_data: &IrData, sample_rate: f32, config: &PluginConfig) -> Vec<f32> {
    let (ir_samples, ir_sample_rate) = decode_ir_samples(&ir_data.raw_bytes);

    let mut resampler = init_resampler(&ir_samples, ir_sample_rate as f64, sample_rate as f64);

    let resampled = &mut resampler.process(&[ir_samples], None).unwrap()[0];

    if config.normalize_irs {
        rms_normalize(resampled, config.normalization_level);
    }
    resampled.clone()
}
