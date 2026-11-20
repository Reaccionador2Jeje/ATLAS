use std::{fs, path::Path};
use serde::{Deserialize, Serialize};
use crate::{error::io_error, AtlasError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AtlasConfig { pub modules_dir: String, pub log_level: String, pub api_bind: String }
impl Default for AtlasConfig {
    fn default() -> Self {
        Self { modules_dir: "modules".into(), log_level: "info".into(), api_bind: "127.0.0.1:7780".into() }
    }
}
impl AtlasConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let source = fs::read_to_string(path).map_err(|e| io_error(path, e))?;
        let config: Self = toml::from_str(&source).map_err(|e| AtlasError::Config(e.to_string()))?;
        if config.modules_dir.trim().is_empty() || config.api_bind.trim().is_empty() {
            return Err(AtlasError::Config("modules_dir and api_bind cannot be empty".into()));
        }
        Ok(config)
    }
}
