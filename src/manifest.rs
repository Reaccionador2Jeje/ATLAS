use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use crate::{AtlasError, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Capability {
    FilesystemRead, FilesystemWrite, NetworkClient, ProcessSpawn,
    SystemInfo, DatabaseRead, DatabaseWrite, SecretAccess,
}
impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_value(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", value.as_str().unwrap_or("unknown"))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleDependency {
    pub id: String,
    pub version: VersionReq,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleManifest {
    pub id: String,
    pub name: String,
    pub version: Version,
    pub api_version: u32,
    pub description: String,
    #[serde(default)]
    pub entrypoint: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
    #[serde(default)]
    pub dependencies: Vec<ModuleDependency>,
}
impl ModuleManifest {
    pub const API_VERSION: u32 = 1;

    pub fn validate(&self) -> Result<()> {
        if !valid_module_id(&self.id) { return Err(AtlasError::InvalidModuleId(self.id.clone())); }
        if self.name.trim().is_empty() || self.description.trim().is_empty() {
            return Err(AtlasError::Config(format!("module {} needs a name and description", self.id)));
        }
        if self.api_version != Self::API_VERSION {
            return Err(AtlasError::Config(format!("module {} targets API {}, supported API is {}", self.id, self.api_version, Self::API_VERSION)));
        }
        if let Some(entrypoint) = &self.entrypoint {
            if entrypoint.starts_with('/') || entrypoint.split('/').any(|part| part == "..") {
                return Err(AtlasError::Config(format!("unsafe entrypoint in module {}", self.id)));
            }
        }
        for dep in &self.dependencies {
            if !valid_module_id(&dep.id) { return Err(AtlasError::InvalidModuleId(dep.id.clone())); }
            if dep.id == self.id { return Err(AtlasError::Dependency(format!("{} depends on itself", self.id))); }
        }
        Ok(())
    }
}
pub fn valid_module_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 96 && id.split('.').all(|segment| {
        !segment.is_empty()
            && segment.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && segment.as_bytes()[0].is_ascii_alphanumeric()
            && segment.as_bytes()[segment.len()-1].is_ascii_alphanumeric()
    })
}
