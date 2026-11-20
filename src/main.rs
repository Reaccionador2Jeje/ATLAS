use std::path::PathBuf;
use clap::{Parser, Subcommand};
use atlas::{config::AtlasConfig, policy::CapabilityPolicy, runtime::ModuleRuntime, ModuleRegistry};

#[derive(Debug, Parser)]
#[command(name = "atlas", version, about = "ATLAS modular technology ecosystem")]
struct Cli {
    #[arg(long, global = true, default_value = "atlas.toml")]
    config: PathBuf,
    #[command(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
enum Command {
    /// Print product and module API versions.
    Version,
    /// Validate configuration and inspect local readiness.
    Doctor,
    /// Inspect the local module catalog.
    Module { #[command(subcommand)] command: ModuleCommand },
}
#[derive(Debug, Subcommand)]
enum ModuleCommand {
    /// List discovered modules.
    List,
    /// Validate dependencies and the default capability policy.
    Plan,
}
fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("atlas: {error}");
        std::process::exit(1);
    }
}
fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Version => {
            println!("ATLAS {}", env!("CARGO_PKG_VERSION"));
            println!("Module API: {}", atlas::ModuleManifest::API_VERSION);
        }
        Command::Doctor => {
            let config = load_config(&cli.config)?;
            println!("ATLAS core: OK");
            println!("Configuration: {}", cli.config.display());
            println!("Modules directory: {} ({})", config.modules_dir, if std::path::Path::new(&config.modules_dir).is_dir() { "present" } else { "not created yet" });
            println!("Module execution: disabled in this foundation release");
        }
        Command::Module { command } => {
            let config = load_config(&cli.config)?;
            let registry = ModuleRegistry::discover(&config.modules_dir)?;
            match command {
                ModuleCommand::List => {
                    if registry.is_empty() { println!("No modules discovered in {}", config.modules_dir); }
                    for (id, module) in registry.iter() {
                        println!("{id:<28} {:<12} {}", module.manifest.version, module.manifest.name);
                    }
                }
                ModuleCommand::Plan => {
                    let mut runtime = ModuleRuntime::default();
                    let order = runtime.prepare(&registry, &CapabilityPolicy::deny_all())?;
                    println!("Validated {} module(s):", order.len());
                    for id in order { println!("  {id}: {}", runtime.state(&id).unwrap()); }
                }
            }
        }
    }
    Ok(())
}
fn load_config(path: &std::path::Path) -> Result<AtlasConfig, Box<dyn std::error::Error>> {
    if path.exists() { Ok(AtlasConfig::load(path)?) } else { Ok(AtlasConfig::default()) }
}
