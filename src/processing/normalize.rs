// TODO: provide option for other normalization, e.g. LUFS

// https://hackaudio.com/tutorial-courses/learn-audio-programming-table-of-contents/digital-signal-processing/amplitude/rms-normalization/

use nih_plug::util::db_to_gain;

pub fn rms_normalize(input_signal: &mut [Vec<f32>], desired_level_db: f32) {
    let desired_level_gain = db_to_gain(desired_level_db);
    for channel in input_signal.iter_mut() {
        let channel_len = channel.len() as f32;

        let squared_sum = channel.iter().map(|x| x * x).sum::<f32>();
        let amplitude = ((channel_len * desired_level_gain.powi(2)) / squared_sum).sqrt();
        println!("Normalizing by factor: {amplitude}");

        channel.iter_mut().for_each(|x| *x *= amplitude);
    }
}
