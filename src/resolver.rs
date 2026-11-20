use std::collections::{BTreeMap, BTreeSet};
use crate::{AtlasError, ModuleRegistry, Result};

/// Deterministic dependency-first order. Fails on missing, incompatible or cyclic dependencies.
pub fn startup_order(registry: &ModuleRegistry) -> Result<Vec<String>> {
    let mut marks = BTreeMap::<String, u8>::new();
    let mut output = Vec::with_capacity(registry.len());
    let mut stack = Vec::new();
    for (id, _) in registry.iter() { visit(id, registry, &mut marks, &mut stack, &mut output)?; }
    Ok(output)
}
fn visit(id: &str, registry: &ModuleRegistry, marks: &mut BTreeMap<String,u8>, stack: &mut Vec<String>, output: &mut Vec<String>) -> Result<()> {
    match marks.get(id).copied().unwrap_or(0) {
        2 => return Ok(()),
        1 => { stack.push(id.to_owned()); return Err(AtlasError::Dependency(format!("dependency cycle: {}", stack.join(" -> ")))); }
        _ => {}
    }
    let module = registry.get(id).ok_or_else(|| AtlasError::ModuleNotFound(id.to_owned()))?;
    marks.insert(id.to_owned(), 1);
    stack.push(id.to_owned());
    let mut deps = module.manifest.dependencies.iter().collect::<Vec<_>>();
    deps.sort_by(|a,b| a.id.cmp(&b.id));
    for dep in deps {
        let target = registry.get(&dep.id).ok_or_else(|| AtlasError::Dependency(format!("{id} requires missing module {}", dep.id)))?;
        if !dep.version.matches(&target.manifest.version) {
            return Err(AtlasError::Dependency(format!("{id} requires {} {}, found {}", dep.id, dep.version, target.manifest.version)));
        }
        visit(&dep.id, registry, marks, stack, output)?;
    }
    stack.pop();
    marks.insert(id.to_owned(), 2);
    output.push(id.to_owned());
    Ok(())
}
pub fn dependency_closure(registry: &ModuleRegistry, roots: &[String]) -> Result<BTreeSet<String>> {
    let mut selected = BTreeSet::new();
    let mut pending = roots.to_vec();
    while let Some(id) = pending.pop() {
        if !selected.insert(id.clone()) { continue; }
        let module = registry.get(&id).ok_or_else(|| AtlasError::ModuleNotFound(id.clone()))?;
        for dep in &module.manifest.dependencies {
            let target = registry.get(&dep.id).ok_or_else(|| AtlasError::Dependency(format!("{id} requires missing module {}", dep.id)))?;
            if !dep.version.matches(&target.manifest.version) {
                return Err(AtlasError::Dependency(format!("{id} requires {} {}, found {}", dep.id, dep.version, target.manifest.version)));
            }
            pending.push(dep.id.clone());
        }
    }
    Ok(selected)
}
