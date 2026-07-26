use crate::catalog::{Category, Repository};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub short_description: String,
    pub detailed_description: String,
    pub categories: Vec<Category>,
    pub tags: Vec<String>,
    pub repository: Repository,
    pub available_version: String,
    pub installed_version: Option<String>,
    pub installed: bool,
    pub licenses: Vec<String>,
    pub homepage: Option<String>,
    pub dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub groups: Vec<String>,
    pub executables: Vec<String>,
    pub related: Vec<String>,
    pub metadata_source: MetadataSource,
    pub packager: Option<String>,
    pub arch: Option<String>,
    pub build_date: Option<i64>,
    pub install_date: Option<i64>,
    pub download_size: Option<i64>,
    pub installed_size: Option<i64>,
    pub filename: Option<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub required_by: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetadataSource {
    Curated,
    BlackArch,
    ArchRepo,
    Imported,
}

impl Tool {
    pub fn display_description(&self) -> &str {
        if self.short_description.is_empty() {
            "No description available"
        } else {
            &self.short_description
        }
    }

    pub fn status_icon(&self) -> &str {
        if self.installed {
            "●"
        } else {
            "○"
        }
    }

    pub fn status_text(&self) -> &str {
        if self.installed {
            "Installed"
        } else {
            "Available"
        }
    }
}
