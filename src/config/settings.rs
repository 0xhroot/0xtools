use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub show_non_security: bool,
    #[serde(default = "default_max_results")]
    pub max_search_results: usize,
    #[serde(default)]
    pub confirm_transactions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            show_non_security: false,
            max_search_results: default_max_results(),
            confirm_transactions: true,
        }
    }
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_max_results() -> usize {
    100
}

impl Config {
    pub fn config_dir() -> Option<PathBuf> {
        directories::ProjectDirs::from("", "", "0xtools")
            .map(|dirs| dirs.config_dir().to_path_buf())
    }

    pub fn load() -> Self {
        let Some(config_dir) = Self::config_dir() else {
            return Self::default();
        };

        let config_path = config_dir.join("config.toml");
        if !config_path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => match toml::from_str::<Config>(&content) {
                Ok(config) => config,
                Err(e) => {
                    tracing::warn!("Failed to parse config: {}, using defaults", e);
                    Self::default()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read config: {}, using defaults", e);
                Self::default()
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let Some(config_dir) = Self::config_dir() else {
            return Err(AppError::Config(
                "Cannot determine config directory".to_string(),
            ));
        };

        fs::create_dir_all(&config_dir)
            .map_err(|e| AppError::Config(format!("Failed to create config dir: {}", e)))?;

        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_path, content)
            .map_err(|e| AppError::Config(format!("Failed to write config: {}", e)))?;

        Ok(())
    }
}
