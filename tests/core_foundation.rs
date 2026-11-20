use std::{fs, path::PathBuf};
use atlas::{manifest::Capability, policy::CapabilityPolicy, resolver::startup_order, runtime::{ModuleRuntime, ModuleState}, ModuleManifest, ModuleRegistry};

fn manifest(id: &str, dependencies: Vec<atlas::ModuleDependency>, capabilities: Vec<Capability>) -> ModuleManifest {
    ModuleManifest {
        id: id.into(), name: id.into(), version: semver::Version::parse("1.2.0").unwrap(),
        api_version: 1, description: "test module".into(), entrypoint: None, capabilities, dependencies,
    }
}
#[test]
fn registry_discovers_and_plans_modules() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().join("system");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("atlas.module.toml"), "id = \"atlas.system\"\nname = \"System\"\nversion = \"1.0.0\"\napi_version = 1\ndescription = \"System module\"\ncapabilities = [\"system-info\"]\ndependencies = []\n").unwrap();
    let registry = ModuleRegistry::discover(temp.path()).unwrap();
    assert_eq!(registry.len(), 1);
    let mut runtime = ModuleRuntime::default();
    let order = runtime.prepare(&registry, &CapabilityPolicy::allow(Capability::SystemInfo)).unwrap();
    assert_eq!(order, vec!["atlas.system"]);
    assert_eq!(runtime.state("atlas.system"), Some(ModuleState::Ready));
}
#[test]
fn resolver_places_dependencies_first() {
    let mut registry = ModuleRegistry::new();
    let dep = atlas::ModuleDependency { id: "atlas.base".into(), version: semver::VersionReq::parse("^1").unwrap() };
    registry.register(PathBuf::from("base"), manifest("atlas.base", vec![], vec![])).unwrap();
    registry.register(PathBuf::from("app"), manifest("atlas.app", vec![dep], vec![])).unwrap();
    assert_eq!(startup_order(&registry).unwrap(), vec!["atlas.base", "atlas.app"]);
}
#[test]
fn policy_denies_ungranted_capabilities() {
    let mut registry = ModuleRegistry::new();
    registry.register(PathBuf::from("system"), manifest("atlas.system", vec![], vec![Capability::NetworkClient])).unwrap();
    assert!(ModuleRuntime::default().prepare(&registry, &CapabilityPolicy::deny_all()).is_err());
}
#[test]
fn runtime_enforces_state_transitions() {
    let mut registry = ModuleRegistry::new();
    registry.register(PathBuf::from("base"), manifest("atlas.base", vec![], vec![])).unwrap();
    let mut runtime = ModuleRuntime::default();
    runtime.prepare(&registry, &CapabilityPolicy::deny_all()).unwrap();
    runtime.start("atlas.base").unwrap();
    assert_eq!(runtime.state("atlas.base"), Some(ModuleState::Running));
    runtime.stop("atlas.base").unwrap();
    assert_eq!(runtime.state("atlas.base"), Some(ModuleState::Stopped));
}
