//! ATLAS core contracts, module discovery, dependency planning and policy checks.
//! This foundation validates modules without executing untrusted code.
pub mod config;
pub mod error;
pub mod event;
pub mod manifest;
pub mod policy;
pub mod registry;
pub mod resolver;
pub mod runtime;

pub use error::{AtlasError, Result};
pub use manifest::{ModuleDependency, ModuleManifest};
pub use registry::ModuleRegistry;
