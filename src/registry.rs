use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};
use crate::{error::io_error, AtlasError, ModuleManifest, Result};

#[derive(Debug, Clone)]
pub struct RegisteredModule {
    pub root: PathBuf,
    pub manifest: ModuleManifest,
}
#[derive(Debug, Default)]
pub struct ModuleRegistry {
    modules: BTreeMap<String, RegisteredModule>,
}
impl ModuleRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, root: PathBuf, manifest: ModuleManifest) -> Result<()> {
        manifest.validate()?;
        if self.modules.contains_key(&manifest.id) { return Err(AtlasError::DuplicateModule(manifest.id)); }
        self.modules.insert(manifest.id.clone(), RegisteredModule { root, manifest });
        Ok(())
    }

    pub fn discover(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref();
        let entries = fs::read_dir(root).map_err(|e| io_error(root, e))?;
        let mut entries = entries.map(|e| e.map_err(|e| io_error(root, e))).collect::<Result<Vec<_>>>()?;
        entries.sort_by_key(|e| e.file_name());
        let mut registry = Self::new();
        for entry in entries {
            let dir = entry.path();
            if !dir.is_dir() { continue; }
            let path = dir.join("atlas.module.toml");
            if !path.is_file() { continue; }
            let source = fs::read_to_string(&path).map_err(|e| io_error(&path, e))?;
            let manifest = toml::from_str(&source).map_err(|e| AtlasError::Manifest { path: path.clone(), message: e.to_string() })?;
            registry.register(dir, manifest)?;
        }
        Ok(registry)
    }

    pub fn get(&self, id: &str) -> Option<&RegisteredModule> { self.modules.get(id) }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &RegisteredModule)> { self.modules.iter() }
    pub fn len(&self) -> usize { self.modules.len() }
    pub fn is_empty(&self) -> bool { self.modules.is_empty() }
}
