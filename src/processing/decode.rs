use hound::SampleFormat;
// TODO: fix precision issues across this module

pub fn decode_samples(bytes: &[u8]) -> anyhow::Result<(Vec<Vec<f32>>, f32)> {
    let mut reader = hound::WavReader::new(bytes)?;

    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;

    let num_channels = spec.channels as usize;

    let samples = match spec.sample_format {
        // this format is fairly rare but we might encounter it,
        // so we need to support it

        // TODO: this might be totally wrong,
        // possibly refer to:
        // https://searchfox.org/mozilla-central/source/dom/media/AudioSampleFormat.h#68-221
        SampleFormat::Float => {
            let mut channels = vec![Vec::new(); num_channels];

            for (i, sample) in reader.samples::<f32>().enumerate() {
                let channel_index = i % num_channels;
                channels[channel_index].push(sample?);
            }
            channels
        }

        // more commonly we will see this
        SampleFormat::Int => {
            let scale_factor = max_value_from_bits(spec.bits_per_sample) as f32;

            let mut channels = vec![Vec::new(); num_channels];

            for (i, sample) in reader.samples::<i32>().enumerate() {
                let channel_index = i % num_channels;
                channels[channel_index].push(sample? as f32 / scale_factor);
            }
            channels
        }
    };

    Ok((samples, sample_rate))
}

fn max_value_from_bits(bit_depth: u16) -> i64 {
    2_i64.pow(u32::from(bit_depth) - 1)
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use hound::{Sample, WavSpec};
    use std::{f32::consts::PI, fs::read, path};
    use tempdir::TempDir;

    use super::{decode_samples, max_value_from_bits};

    #[test]
    fn samples_16_bit() -> anyhow::Result<()> {
        write_then_decode_with_bits(16)
    }
    #[test]
    fn samples_24_bit() -> anyhow::Result<()> {
        write_then_decode_with_bits(24)
    }
    #[test]
    fn samples_32_bit() -> anyhow::Result<()> {
        write_then_decode_with_bits(32)
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

        // write
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut samples = Vec::new();
        let len = 100;
        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            samples.push(sample);
        }
        write_samples(&file_name, spec, &samples)?;

        // read
        let buf = read(&file_name)?;
        let (result_samples, _) = decode_samples(&buf)?;
        assert_eq!(samples, result_samples[0]);

        temp_dir.close()?;
        Ok(())
    }

    #[test]
    fn decode_multi_channel() -> anyhow::Result<()> {
        let temp_dir = TempDir::new("wav_testing")?;
        let file_name = temp_dir.path().join("multi_channel.wav");

        let spec = WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let len = 100;
        let mut samples_to_write = Vec::new();
        let mut cmp_samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let scale_factor = max_value_from_bits(spec.bits_per_sample) as f32;

            samples_to_write.push((sample * scale_factor) as i32);
            samples_to_write.push(0);

            cmp_samples.push(sample);
        }

        write_samples(&file_name, spec, &samples_to_write)?;

        // decode
        let file_bytes = read(&file_name)?;
        let res = decode_samples(&file_bytes)?.0;
        let result_samples = &res[0];
        let zero_samples = &res[1];

        for (original_sample, res_sample) in cmp_samples.iter().zip(result_samples) {
            // println!("{original_sample}, {res_sample}");
            assert!(approx_eq!(
                f32,
                *original_sample,
                *res_sample,
                epsilon = (max_value_from_bits(16) as f32).recip()
            ));
        }
        assert_eq!(*zero_samples, vec![0.0f32; result_samples.len()]);

        temp_dir.close()?;
        Ok(())
    }

    fn write_then_decode_with_bits(bit_depth: u16) -> anyhow::Result<()> {
        // write
        let temp_dir = TempDir::new("wav_testing")?;
        let file_name = temp_dir.path().join("test_sine.wav");

        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: bit_depth,
            sample_format: hound::SampleFormat::Int,
        };

        let len = 100;
        let mut samples_to_write = Vec::new();
        let mut cmp_samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let scale_factor = max_value_from_bits(spec.bits_per_sample) as f32;

            samples_to_write.push((sample * scale_factor) as i32);
            cmp_samples.push(sample);
        }

        write_samples(&file_name, spec, &samples_to_write)?;

        // decode
        let file_bytes = read(&file_name)?;
        let decode_result = decode_samples(&file_bytes)?.0;
        let res_samples = &decode_result[0];

        assert!(decode_result.len() == 1);

        for (original_sample, res_sample) in cmp_samples.iter().zip(res_samples) {
            // println!("{original_sample}, {res_sample}");
            assert!(approx_eq!(
                f32,
                *original_sample,
                *res_sample,
                epsilon = (max_value_from_bits(bit_depth) as f32).recip()
            ));
        }

        temp_dir.close()?;
        Ok(())
    }

    fn write_samples<P, T>(name: P, spec: WavSpec, samples: &Vec<T>) -> anyhow::Result<()>
    where
        P: AsRef<path::Path>,
        T: Sample + Clone,
    {
        let mut writer = hound::WavWriter::create(name, spec)?;
        for s in samples {
            writer.write_sample(s.clone())?;
        }
        writer.finalize()?;
        Ok(())
    }
}
