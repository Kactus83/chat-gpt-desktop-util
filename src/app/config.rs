use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub some_setting: bool,
    // Ajoutez ici d'autres paramètres de configuration
}

impl Config {
    pub fn new() -> Self {
        Self {
            some_setting: true,
            // Initialisez les autres paramètres ici
        }
    }

    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            let config_data = fs::read_to_string(config_path).expect("Failed to read config file");
            serde_json::from_str(&config_data).expect("Failed to parse config file")
        }
    }
}
