use hound::SampleFormat;
use nih_plug::util::db_to_gain;

// NOTE: right now this only handles mono
pub fn decode_ir_samples(bytes: &[u8]) -> anyhow::Result<(Vec<f32>, f32)> {
    let mut reader = hound::WavReader::new(bytes)?;

    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;

    let samples = match spec.sample_format {
        // this format is fairly rare but we might encounter it,
        // so we need to support it
        SampleFormat::Float => reader
            .samples::<f32>()
            .map(|x| Ok(x?))
            .collect::<anyhow::Result<Vec<f32>>>()?,
        SampleFormat::Int => {
            let bit_depth = spec.bits_per_sample as u32;
            // 1 << (bit_depth - 1) might be correct
            let max_amplitude = 2_i64.pow(bit_depth - 1) as f32;

            reader
                .samples::<i32>()
                .map(|x| Ok(x? as f32 / max_amplitude))
                .collect::<anyhow::Result<Vec<f32>>>()?
        }
    };

    Ok((samples, sample_rate))
}

// first attempt was peak normalization, didn't work very well for a variety of irs
// https://hackaudio.com/tutorial-courses/learn-audio-programming-table-of-contents/digital-signal-processing/amplitude/rms-normalization/

pub fn rms_normalize(input_signal: &mut [f32], desired_level_db: f32) {
    let input_len = input_signal.len() as f32;
    let desired_level_gain = db_to_gain(desired_level_db);

    let squared_sum = input_signal.iter().map(|x| x * x).sum::<f32>();

    let amplitude = ((input_len * desired_level_gain.powi(2)) / squared_sum).sqrt();
    println!("Normalizing by factor: {amplitude}");

    input_signal.iter_mut().for_each(|x| *x *= amplitude);
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::PI, fs::read, path};

    use float_cmp::approx_eq;
    use tempdir::TempDir;

    use crate::util::decode_ir_samples;

    // test function
    // this writes a file AND returns an array of the samples
    // then the read function can be tested by comparing samples

    fn write_test_file<P>(name: P, num_samples: usize) -> anyhow::Result<Vec<f32>>
    where
        P: AsRef<path::Path>,
    {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Int,
        };

        let len = num_samples;

        let mut writer = hound::WavWriter::create(name, spec)?;
        let mut samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            samples.push(sample);
            let amplitude = i32::MAX as f32;
            writer.write_sample((sample * amplitude) as i32)?;
        }
        writer.finalize()?;

        Ok(samples)
    }
    // TODO: it might be possible to refactor these functions to be more similar
    fn write_test_file_f32<P>(name: P, len: usize) -> anyhow::Result<Vec<f32>>
    where
        P: AsRef<path::Path>,
    {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = hound::WavWriter::create(name, spec).unwrap();
        let mut samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            samples.push(sample);
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();

        Ok(samples)
    }

    #[test]
    fn decode_samples() -> anyhow::Result<()> {
        let temp_dir = TempDir::new("wav_testing")?;
        let file_name = temp_dir.path().join("test_sine.wav");
        let samples = write_test_file(&file_name, 100)?;

        let buf = read(&file_name)?;
        let (other, _) = decode_ir_samples(&buf).unwrap();

        for (l, r) in samples.iter().zip(other) {
            // TODO: find proper epsilon
            assert!(approx_eq!(f32, *l, r, epsilon = 0.0000001))
        }

        temp_dir.close()?;
        Ok(())
    }
    #[test]
    fn decode_samples_f32() -> anyhow::Result<()> {
        let temp_dir = TempDir::new("wav_testing")?;
        let file_name = temp_dir.path().join("test_sine.wav");
        let original_samples = write_test_file_f32(&file_name, 100)?;

        let buf = read(&file_name)?;
        let (result_samples, _) = decode_ir_samples(&buf).unwrap();

        assert_eq!(original_samples, result_samples);

        temp_dir.close()?;
        Ok(())
    }

    // TODO: make this stupid test pass
    /*
    #[test]
    fn test_normalize() {
        let mut samples = read_samples_from_file("test_irs\\vsmall.wav");

        let desired_rms = -18.0f32;
        rms_normalize(&mut samples, desired_rms);

        let n = samples.len() as f32;
        let new_rms = (samples.iter().map(|x| x.powi(2)).sum::<f32>() / n).sqrt();

        assert_eq!(gain_to_db(new_rms), desired_rms);
    }*/
}
