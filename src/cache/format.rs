use crate::catalog::tool::Tool;
use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

const CACHE_VERSION: u32 = 1;
const CACHE_MAX_AGE_SECS: u64 = 3600 * 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub version: u32,
    pub tools: Vec<Tool>,
    pub blackarch_detected: bool,
    pub tool_count: usize,
    pub installed_count: usize,
    pub blackarch_count: usize,
    pub created_at: String,
}

impl CacheData {
    pub fn new(tools: Vec<Tool>, blackarch_detected: bool) -> Self {
        let tool_count = tools.len();
        let installed_count = tools.iter().filter(|t| t.installed).count();
        let blackarch_count = tools
            .iter()
            .filter(|t| t.repository == crate::catalog::Repository::BlackArch)
            .count();

        let now = chrono::Utc::now().to_rfc3339();

        Self {
            version: CACHE_VERSION,
            tools,
            blackarch_detected,
            tool_count,
            installed_count,
            blackarch_count,
            created_at: now,
        }
    }
}

pub struct CacheStore {
    base_dir: PathBuf,
}

impl CacheStore {
    pub fn new() -> Result<Self> {
        let dirs = directories::ProjectDirs::from("", "", "0xtools")
            .ok_or_else(|| AppError::Cache("Cannot determine cache directory".to_string()))?;

        let cache_dir = dirs.cache_dir().to_path_buf();
        fs::create_dir_all(&cache_dir)
            .map_err(|e| AppError::Cache(format!("Failed to create cache dir: {}", e)))?;

        Ok(Self {
            base_dir: cache_dir,
        })
    }

    fn tools_path(&self) -> PathBuf {
        self.base_dir.join("tools.bin")
    }

    fn meta_path(&self) -> PathBuf {
        self.base_dir.join("meta.toml")
    }

    fn tmp_path(&self) -> PathBuf {
        self.base_dir.join("tools.bin.tmp")
    }

    pub fn save(&self, data: &CacheData) -> Result<()> {
        let serialized = bincode::serialize(data)
            .map_err(|e| AppError::Cache(format!("Failed to serialize cache: {}", e)))?;

        let tmp = self.tmp_path();
        fs::write(&tmp, &serialized)
            .map_err(|e| AppError::Cache(format!("Failed to write temp cache: {}", e)))?;

        fs::rename(&tmp, self.tools_path())
            .map_err(|e| AppError::Cache(format!("Failed to atomic-replace cache: {}", e)))?;

        let meta = format!(
            "version = {}\ntool_count = {}\nblackarch = {}\n",
            data.version, data.tool_count, data.blackarch_detected
        );
        let _ = fs::write(self.meta_path(), meta);

        tracing::info!(
            "Cache saved: {} tools ({} installed, {} blackarch)",
            data.tool_count,
            data.installed_count,
            data.blackarch_count
        );

        Ok(())
    }

    pub fn load(&self) -> Result<Option<CacheData>> {
        let path = self.tools_path();
        if !path.exists() {
            return Ok(None);
        }

        let meta_path = self.meta_path();
        if meta_path.exists() {
            if let Ok(meta_str) = fs::read_to_string(&meta_path) {
                if meta_str.contains("version = 0") {
                    let _ = fs::remove_file(&path);
                    return Ok(None);
                }
            }
        }

        let bytes =
            fs::read(&path).map_err(|e| AppError::Cache(format!("Failed to read cache: {}", e)))?;

        match bincode::deserialize::<CacheData>(&bytes) {
            Ok(mut data) => {
                if data.version != CACHE_VERSION {
                    tracing::warn!(
                        "Cache version mismatch: {} != {}, rebuilding",
                        data.version,
                        CACHE_VERSION
                    );
                    let _ = fs::remove_file(&path);
                    return Ok(None);
                }
                data.tools.shrink_to_fit();
                Ok(Some(data))
            }
            Err(e) => {
                tracing::warn!("Cache corrupted: {}, will rebuild", e);
                let _ = fs::remove_file(&path);
                Err(AppError::CacheCorrupted)
            }
        }
    }

    pub fn is_stale(&self) -> bool {
        let path = self.tools_path();
        match fs::metadata(&path) {
            Ok(meta) => {
                let modified = meta.modified().unwrap_or_else(|_| SystemTime::now());
                let age = SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or(Duration::from_secs(0));
                age > Duration::from_secs(CACHE_MAX_AGE_SECS)
            }
            Err(_) => true,
        }
    }

    pub fn invalidate(&self) -> Result<()> {
        let _ = fs::remove_file(self.tools_path());
        let _ = fs::remove_file(self.meta_path());
        Ok(())
    }

    pub fn exists(&self) -> bool {
        self.tools_path().exists()
    }
}
