use std::collections::BTreeMap;
use crate::{policy::CapabilityPolicy, resolver::startup_order, AtlasError, ModuleRegistry, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState { Ready, Running, Stopped }
impl std::fmt::Display for ModuleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self { Self::Ready => "ready", Self::Running => "running", Self::Stopped => "stopped" })
    }
}
/// Lifecycle state coordinator only; it does not load or execute module code.
#[derive(Debug, Default)]
pub struct ModuleRuntime { states: BTreeMap<String, ModuleState> }
impl ModuleRuntime {
    pub fn prepare(&mut self, registry: &ModuleRegistry, policy: &CapabilityPolicy) -> Result<Vec<String>> {
        let order = startup_order(registry)?;
        for id in &order {
            let module = registry.get(id).ok_or_else(|| AtlasError::ModuleNotFound(id.clone()))?;
            policy.check(&module.manifest)?;
            self.states.insert(id.clone(), ModuleState::Ready);
        }
        Ok(order)
    }
    pub fn start(&mut self, id: &str) -> Result<()> {
        let state = self.states.get(id).copied().ok_or_else(|| AtlasError::ModuleNotFound(id.into()))?;
        if state != ModuleState::Ready && state != ModuleState::Stopped {
            return Err(AtlasError::InvalidTransition { module: id.into(), from: state.to_string(), to: "running".into() });
        }
        self.states.insert(id.into(), ModuleState::Running);
        Ok(())
    }
    pub fn stop(&mut self, id: &str) -> Result<()> {
        let state = self.states.get(id).copied().ok_or_else(|| AtlasError::ModuleNotFound(id.into()))?;
        if state != ModuleState::Running {
            return Err(AtlasError::InvalidTransition { module: id.into(), from: state.to_string(), to: "stopped".into() });
        }
        self.states.insert(id.into(), ModuleState::Stopped);
        Ok(())
    }
    pub fn state(&self, id: &str) -> Option<ModuleState> { self.states.get(id).copied() }
}
