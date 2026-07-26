use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub tool: String,
    pub overview: String,
    pub purpose: String,
    pub common_options: Vec<CmdOption>,
    pub examples: Vec<String>,
    pub official_docs: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdOption {
    pub flag: String,
    pub description: String,
}

pub struct ReferenceStore {
    references: HashMap<String, Reference>,
}

impl ReferenceStore {
    pub fn load_from_dir(dir: &Path) -> Self {
        let mut references = HashMap::new();

        if dir.exists() {
            for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(reference) = toml::from_str::<Reference>(&content) {
                            references.insert(reference.tool.clone(), reference);
                        }
                    }
                }
            }
        }

        Self { references }
    }

    pub fn get(&self, tool_name: &str) -> Option<&Reference> {
        self.references.get(tool_name)
    }

    pub fn has(&self, tool_name: &str) -> bool {
        self.references.contains_key(tool_name)
    }

    pub fn available_tools(&self) -> Vec<&str> {
        self.references.keys().map(|s| s.as_str()).collect()
    }
}
