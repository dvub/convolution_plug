use nih_plug::util::db_to_gain;

pub fn decode_ir_samples(bytes: &[u8]) -> anyhow::Result<(Vec<f32>, f32)> {
    let mut reader = hound::WavReader::new(bytes)?;

    let bit_depth = reader.spec().bits_per_sample as u32;
    let sample_rate = reader.spec().sample_rate as f32;

    let max_amplitude = 2_i32.pow(bit_depth - 1) as f32;

    let samples = reader
        .samples::<i32>()
        .map(|s| s.unwrap_or(0) as f32 / max_amplitude)
        .collect();

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
    use std::{
        f32::consts::PI,
        fs::{read, remove_file},
    };

    use float_cmp::approx_eq;

    use crate::util::decode_ir_samples;

    // test function
    // this writes a file AND returns an array of the samples
    // then the read function can be tested by comparing samples
    fn write_test_file(name: &str, num_samples: usize) -> Vec<f32> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let len = num_samples;

        let mut writer = hound::WavWriter::create(name, spec).unwrap();
        let mut samples = Vec::new();

        for t in (0..len).map(|x| x as f32 / (len as f32)) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            samples.push(sample);
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
        writer.finalize().unwrap();

        samples
    }

    // TODO: make this test pass without manually checking
    #[test]
    fn decode_samples() {
        // TODO: use better name and proper temp directory
        let file_name = "sine.wav";

        let samples = write_test_file(file_name, 100);

        let buf = read(file_name).unwrap();

        // println!("{:?}", buf);
        let (other, _) = decode_ir_samples(&buf).unwrap();

        // this might be horrible
        remove_file(file_name).unwrap();

        assert_eq!(samples, other);
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
