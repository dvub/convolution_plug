use rubato::{SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};

// TODO: make this not bad

const RESAMPLING_CHANNELS: usize = 1;

pub fn init_resampler(
    ir_samples: &[f32],
    ir_sample_rate: f64,
    desired_sample_rate: f64,
) -> anyhow::Result<SincFixedIn<f32>> {
    let resampling_params = SincInterpolationParameters {
        sinc_len: 384,
        f_cutoff: 1.0,
        interpolation: SincInterpolationType::Cubic,
        oversampling_factor: 128,
        window: WindowFunction::Hann,
    };
    Ok(SincFixedIn::<f32>::new(
        desired_sample_rate / ir_sample_rate,
        10.0,
        resampling_params,
        ir_samples.len(),
        RESAMPLING_CHANNELS,
    )?)
}
