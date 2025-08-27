mod freq_response;
pub mod spectrum_analyzer;

use std::sync::Arc;

use crossbeam_channel::{bounded, Sender};
use nih_plug::prelude::AtomicF32;

use crate::{
    editor::eq::{freq_response::FrequencyResponse, spectrum_analyzer::SpectrumAnalyzer},
    params::PluginParams,
};

pub struct Equalizer {
    dry_spectrum: SpectrumAnalyzer,
    wet_spectrum: SpectrumAnalyzer,
    frequency_response: FrequencyResponse,
    params: Arc<PluginParams>,
    pub dry_tx: Sender<f32>,
    pub wet_tx: Sender<f32>,
}
impl Equalizer {
    pub fn new(sample_rate: Arc<AtomicF32>, params: Arc<PluginParams>) -> Self {
        let (dry_tx, dry_rx) = bounded(1024);
        let (wet_tx, wet_rx) = bounded(1024);

        Self {
            dry_spectrum: SpectrumAnalyzer::new(sample_rate.clone(), dry_rx.clone()),
            wet_spectrum: SpectrumAnalyzer::new(sample_rate.clone(), wet_rx.clone()),
            frequency_response: FrequencyResponse::new(params.clone(), sample_rate.clone()),
            params: params.clone(),
            dry_tx,
            wet_tx,
        }
    }
}
