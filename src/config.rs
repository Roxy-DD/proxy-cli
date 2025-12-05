use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub enabled: bool,
    pub port: Option<u16>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialize/deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid port: {0} (must be 1-65535)")]
    InvalidPort(u32),
}

pub fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("proxy-cli")
        .join("config.json")
}

fn init_config_dir(config_path: &Path) -> Result<(), ConfigError> {
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

pub fn load_config() -> Result<Config, ConfigError> {
    let config_path = get_config_path();
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let mut config: Config = serde_json::from_str(&content)?;
        if let Some(port) = config.port {
            if port < 1 {
                config.port = None;
                save_config(&config)?;
            }
        }
        Ok(config)
    } else {
        let default_config = Config::default();
        save_config(&default_config)?;
        Ok(default_config)
    }
}

pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    let config_path = get_config_path();
    init_config_dir(&config_path)?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}

pub fn validate_port(port: u32) -> Result<u16, ConfigError> {
    if port >= 1 && port <= 65535 {
        Ok(port as u16)
    } else {
        Err(ConfigError::InvalidPort(port))
    }
}