//! Provides basic, configurable smoothing for FFT bins.

// code ported from FunDSP:
// https://github.com/SamiPerttu/fundsp/blob/a4f126bcbb5c6b93c4cd65662035655913e1e830/src/dynamics.rs#L343

// this is a stripped-down version of meter/monitor stuff from fundsp

use serde::{Deserialize, Serialize};
use ts_rs::TS;

const DEFAULT_FPS: f32 = 45.0;

#[derive(Deserialize, Serialize, TS, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[ts(export)]
pub enum MonitorMode {
    Sample,
    Peak(f32),
    Rms(f32),
}

#[derive(Clone)]
pub struct Monitor {
    meter: MonitorMode,
    state: f32,
    smoothing: f32,
    fps: f32,
}

impl Monitor {
    /// Create a new Monitor based on the given mode.
    pub fn new(meter: MonitorMode) -> Self {
        let mut monitor = Self {
            meter,
            state: 0.0,
            smoothing: 0.0,
            fps: 0.0,
        };

        monitor.set_frame_rate(DEFAULT_FPS);
        monitor
    }

    /// Set the frame rate at which the meter is updated.
    pub fn set_frame_rate(&mut self, frame_rate: f32) {
        let timescale = match self.meter {
            MonitorMode::Sample => {
                return;
            }
            MonitorMode::Peak(timescale) => timescale,
            MonitorMode::Rms(timescale) => timescale,
        };
        self.smoothing = 0.5f32.powf(1.0 / (timescale * frame_rate));
        self.fps = frame_rate;
    }
    /*
    pub fn set_mode(&mut self, meter: MonitorMode) {
        self.meter = meter;
    }
    pub fn set_decay_speed(&mut self, new_speed: f32) {
        match self.meter {
            MonitorMode::Sample => {
                return;
            }
            MonitorMode::Peak(_) => self.meter = MonitorMode::Peak(new_speed),
            MonitorMode::Rms(_) => self.meter = MonitorMode::Rms(new_speed),
        };
        self.smoothing = 0.5f32.powf(1.0 / (new_speed * self.fps));
    } */

    /// Process an input value.
    pub fn tick(&mut self, value: f32) {
        match self.meter {
            MonitorMode::Sample => self.state = value,
            MonitorMode::Peak(_) => self.state = f32::max(self.state * self.smoothing, value.abs()),
            MonitorMode::Rms(_) => {
                self.state = self.state * self.smoothing + value.powi(2) * (1.0 - self.smoothing)
            }
        }
    }

    pub fn level(&self) -> f32 {
        match self.meter {
            MonitorMode::Sample => self.state,
            MonitorMode::Peak(_) => self.state,
            MonitorMode::Rms(_) => self.state.sqrt(),
        }
    }
}
