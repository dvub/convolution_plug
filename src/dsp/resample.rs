use rubato::{SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};

const RESAMPLING_CHANNELS: usize = 2;

pub fn init_resampler(
    len: usize,
    ir_sample_rate: f64,
    desired_sample_rate: f64,
) -> anyhow::Result<SincFixedIn<f32>> {
    // TODO: maybe make more of these parameters consts?
    // might make code more readable, not sure
    let resampling_params = SincInterpolationParameters {
        sinc_len: 384,
        f_cutoff: 0.97, // i.. chose this number completely randomly, tbh
        interpolation: SincInterpolationType::Cubic,
        oversampling_factor: 256,
        window: WindowFunction::Hann,
    };
    Ok(SincFixedIn::<f32>::new(
        desired_sample_rate / ir_sample_rate,
        1.0, // we're not changing the ratio so this doesn't matter right now
        resampling_params,
        len,
        RESAMPLING_CHANNELS,
    )?)
}
