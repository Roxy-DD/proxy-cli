use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 7890, // Default port
            enabled: false,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        let proj_dirs = ProjectDirs::from("com", "proxy-cli", "proxy-cli")
            .expect("Could not determine config directory");
        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).expect("Failed to create config directory");
        }

        Self {
            config_path: config_dir.join("config.json"),
        }
    }

    pub fn load(&self) -> AppConfig {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)
    }
}
