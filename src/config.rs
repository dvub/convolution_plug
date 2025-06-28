use nih_plug::nih_log;
use serde::{Deserialize, Serialize};

use directories::ProjectDirs;

use serde_json::json;
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
};

pub const DEFAULT_NORMALIZATION_LEVEL: f32 = -48.0;
pub const DEFAULT_FADE_TIME: f64 = 1.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    pub default_ir_path: String,
    pub normalize_irs: bool,
    pub normalization_level: f32,
    pub fade_time: f64,
}

#[allow(clippy::derivable_impls)]
impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            default_ir_path: String::new(),
            normalize_irs: true,
            normalization_level: DEFAULT_NORMALIZATION_LEVEL,
            fade_time: DEFAULT_FADE_TIME,
        }
    }
}

impl PluginConfig {
    // TODO:
    // write testing for this feature

    pub fn get_config() -> anyhow::Result<PluginConfig> {
        let project_dir = ProjectDirs::from("com", "dvub", "convolution_plug")
            .expect("Couldn't find a valid home directory. This is not a plugin issue.");

        let config_dir = project_dir.config_dir();
        let config_path = config_dir.join("settings.json");

        if !config_dir.exists() {
            println!("config directory doesn't exist, creating it now");
            create_dir_all(config_dir)?;
        }

        // write a default config file if it doesn't exist
        if !config_path.exists() {
            nih_log!("settings.json doesn't exist, writing a default config.");

            let mut file = File::create(&config_path)?;

            let default_config = PluginConfig::default();
            let config_str = json!(default_config).to_string();

            file.write_all(config_str.as_bytes())?;
        } else {
            nih_log!("Found an existing settings.json, attempting to read it now");
        }

        // we can expect() here because we've ensured it was created right before this
        let contents = read_to_string(&config_path)
            .expect("Could not find settings.json when it should exist.");

        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
