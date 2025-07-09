use hound::SampleFormat;
use nih_plug::util::db_to_gain;

// NOTE: right now this only handles mono
pub fn decode_ir_samples(bytes: &[u8]) -> anyhow::Result<(Vec<f32>, f32)> {
    let mut reader = hound::WavReader::new(bytes)?;

    let spec = reader.spec();

    let sample_rate = spec.sample_rate as f32;

    let samples: anyhow::Result<Vec<f32>> = match spec.sample_format {
        // this format is fairly rare but we might encounter it,
        // so we need to support it

        // TODO: this might be totally wrong,
        // possibly refer to:
        // https://searchfox.org/mozilla-central/source/dom/media/AudioSampleFormat.h#68-221
        SampleFormat::Float => reader.samples::<f32>().map(|x| Ok(x?)).collect(),
        SampleFormat::Int => {
            let bit_depth = spec.bits_per_sample;

            let scale_factor = max_value_from_bits(bit_depth) as f32;

            reader
                .samples::<i32>()
                .map(|x| Ok(x? as f32 / scale_factor))
                .collect()
        }
    };

    Ok((samples?, sample_rate))
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

fn max_value_from_bits(bit_depth: u16) -> i64 {
    2_i64.pow(bit_depth as u32 - 1)
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::PI, fs::read, path};

    use float_cmp::approx_eq;

    use hound::WavSpec;
    use tempdir::TempDir;

    use crate::util::{decode_ir_samples, max_value_from_bits};

    #[test]
    fn samples_16_bit() -> anyhow::Result<()> {
        decode_samples_with_bits(16)?;
        Ok(())
    }
    #[test]
    fn samples_24_bit() -> anyhow::Result<()> {
        decode_samples_with_bits(24)?;
        Ok(())
    }
    #[test]
    fn samples_32_bit() -> anyhow::Result<()> {
        decode_samples_with_bits(32)?;
        Ok(())
    }

    #[test]
    fn sanity() {
        assert_eq!((max_value_from_bits(8) - 1) as i8, i8::MAX);
        assert_eq!((max_value_from_bits(16) - 1) as i16, i16::MAX);
        assert_eq!((max_value_from_bits(32) - 1) as i32, i32::MAX);
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

    fn decode_samples_with_bits(bit_depth: u16) -> anyhow::Result<()> {
        let temp_dir = TempDir::new("wav_testing")?;
        let file_name = temp_dir.path().join("test_sine.wav");
        let num_samples = 100;

        let original_samples = write_test_file(
            &file_name,
            num_samples,
            WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: bit_depth,
                sample_format: hound::SampleFormat::Int,
            },
        )?;

        let buf = read(&file_name)?;
        let (result_samples, _) = decode_ir_samples(&buf).unwrap();

        for (original_sample, res_sample) in original_samples.iter().zip(result_samples) {
            println!("{original_sample}, {res_sample}");

            assert!(approx_eq!(
                f32,
                *original_sample,
                res_sample,
                epsilon = (max_value_from_bits(bit_depth) as f32).recip()
            ))
        }

        temp_dir.close()?;
        Ok(())
    }

    fn write_test_file<P>(name: P, num_samples: usize, spec: WavSpec) -> anyhow::Result<Vec<f32>>
    where
        P: AsRef<path::Path>,
    {
        let len = num_samples;

        let mut writer = hound::WavWriter::create(name, spec)?;
        let mut samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            samples.push(sample);

            let amplitude = (1 << (spec.bits_per_sample - 1)) as f32;

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
}
