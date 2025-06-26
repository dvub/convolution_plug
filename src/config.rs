use serde::{Deserialize, Serialize};

use directories::ProjectDirs;

use serde_json::json;
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
};

use crate::util::DEFAULT_NORMALIZATION_LEVEL;

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    pub default_ir_path: String,
    pub normalize_irs: bool,
    pub normalization_level: f32,
}

#[allow(clippy::derivable_impls)]
impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            default_ir_path: String::new(),
            normalize_irs: true,
            normalization_level: DEFAULT_NORMALIZATION_LEVEL,
        }
    }
}

// TODO:
// should this be in impl Config?
// fix unwraps
// write testing for this feature
pub fn get_plugin_config() -> PluginConfig {
    let project_dir = ProjectDirs::from("com", "dvub", "convolution_plug").unwrap();
    let config_dir = project_dir.config_dir();

    if !config_dir.exists() {
        println!("config directory doesn't exist, creating it now");
        create_dir_all(config_dir).unwrap();
    }

    let config_path = config_dir.join("settings.json");

    // write a default config file if it doesn't exist
    if !config_path.exists() {
        println!("Config doesn't exist, writing a new one");
        let mut file = File::create(&config_path).unwrap();
        let default_config = PluginConfig::default();
        let str_config = json!(default_config).to_string();
        file.write_all(str_config.as_bytes()).unwrap();
    } else {
        println!("Found an existing config");
    }
    // read file contents as a Config
    let contents = read_to_string(&config_path).unwrap();
    serde_json::from_str(&contents).unwrap()
}
