use serde::{Deserialize, Serialize};

// this seems insane and this feature probably doesn't properly work
pub const DEFAULT_NORMALIZATION_LEVEL: f32 = -52.0;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginConfig {
    pub normalize_irs: bool,
    pub resample: bool,
    pub normalization_level: f32,
}

#[allow(clippy::derivable_impls)]
impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            normalize_irs: false,
            normalization_level: DEFAULT_NORMALIZATION_LEVEL,
            resample: true,
        }
    }
}
