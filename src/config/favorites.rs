use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Favorites {
    pub packages: Vec<String>,
}

impl Favorites {
    pub fn config_dir() -> Option<PathBuf> {
        directories::ProjectDirs::from("", "", "0xtools")
            .map(|dirs| dirs.config_dir().to_path_buf())
    }

    fn favorites_path() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("favorites.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::favorites_path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
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

        let path = config_dir.join("favorites.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize favorites: {}", e)))?;

        fs::write(&path, content)
            .map_err(|e| AppError::Config(format!("Failed to write favorites: {}", e)))?;

        Ok(())
    }

    pub fn is_favorite(&self, package_name: &str) -> bool {
        self.packages.iter().any(|p| p == package_name)
    }

    pub fn toggle(&mut self, package_name: &str) -> bool {
        if self.is_favorite(package_name) {
            self.packages.retain(|p| p != package_name);
            false
        } else {
            self.packages.push(package_name.to_string());
            true
        }
    }

    pub fn add(&mut self, package_name: &str) {
        if !self.is_favorite(package_name) {
            self.packages.push(package_name.to_string());
        }
    }

    pub fn remove(&mut self, package_name: &str) {
        self.packages.retain(|p| p != package_name);
    }

    pub fn as_set(&self) -> HashSet<String> {
        self.packages.iter().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.packages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle() {
        let mut fav = Favorites::default();
        assert!(!fav.is_favorite("nmap"));
        fav.toggle("nmap");
        assert!(fav.is_favorite("nmap"));
        fav.toggle("nmap");
        assert!(!fav.is_favorite("nmap"));
    }
}
