use std::sync::{atomic::Ordering, Arc};

use nih_plug::prelude::AtomicF32;

use crate::editor::util::DEFAULT_FREQ_RANGE;

pub struct FrequencyResponseConfig {
    pub frequency_range: (f32, f32),
}

impl Default for FrequencyResponseConfig {
    fn default() -> Self {
        Self {
            frequency_range: DEFAULT_FREQ_RANGE,
        }
    }
}

impl FrequencyResponseConfig {
    pub fn new(sample_rate: Arc<AtomicF32>) -> Self {
        Self {
            frequency_range: (DEFAULT_FREQ_RANGE.0, sample_rate.load(Ordering::Relaxed)),
        }
    }
}
