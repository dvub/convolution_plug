use serde::{Deserialize, Serialize};
use ts_rs::TS;

// this seems insane and this feature probably doesn't properly work
pub const DEFAULT_NORMALIZATION_LEVEL: f32 = -46.0;

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct IrProcessingConfig {
    pub normalize: bool,
    pub resample: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for IrProcessingConfig {
    fn default() -> Self {
        Self {
            normalize: false,
            resample: false,
        }
    }
}
