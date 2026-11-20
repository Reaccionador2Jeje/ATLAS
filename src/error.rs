use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum AtlasError {
    #[error("I/O error at {path}: {source}")]
    Io { path: PathBuf, #[source] source: std::io::Error },
    #[error("invalid manifest at {path}: {message}")]
    Manifest { path: PathBuf, message: String },
    #[error("invalid module identifier: {0}")]
    InvalidModuleId(String),
    #[error("duplicate module identifier: {0}")]
    DuplicateModule(String),
    #[error("module not found: {0}")]
    ModuleNotFound(String),
    #[error("dependency error: {0}")]
    Dependency(String),
    #[error("policy denied {module}: capability {capability} is not allowed")]
    CapabilityDenied { module: String, capability: String },
    #[error("invalid lifecycle transition for {module}: {from} -> {to}")]
    InvalidTransition { module: String, from: String, to: String },
    #[error("configuration error: {0}")]
    Config(String),
}
pub type Result<T> = std::result::Result<T, AtlasError>;

pub(crate) fn io_error(path: impl Into<PathBuf>, source: std::io::Error) -> AtlasError {
    AtlasError::Io { path: path.into(), source }
}
