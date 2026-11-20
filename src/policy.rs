use std::collections::BTreeSet;
use crate::{manifest::Capability, AtlasError, ModuleManifest, Result};

#[derive(Debug, Clone, Default)]
pub struct CapabilityPolicy { allowed: BTreeSet<String> }
impl CapabilityPolicy {
    pub fn deny_all() -> Self { Self::default() }
    pub fn allow(mut self, capability: Capability) -> Self {
        self.allowed.insert(capability.to_string());
        self
    }
    pub fn check(&self, manifest: &ModuleManifest) -> Result<()> {
        for capability in &manifest.capabilities {
            let name = capability.to_string();
            if !self.allowed.contains(&name) {
                return Err(AtlasError::CapabilityDenied { module: manifest.id.clone(), capability: name });
            }
        }
        Ok(())
    }
}
