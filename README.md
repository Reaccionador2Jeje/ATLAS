# ATLAS

**ATLAS** is an open-source modular technology ecosystem developed by AtDev. Its goal is to provide shared foundations for software development, system management, infrastructure, data and extensible digital services.

This repository starts with the **ATLAS Core Foundation v0.1**: a Rust core library, module manifest contract, registry, dependency planner, deny-by-default capability policy, lifecycle coordinator, bounded event queue and CLI.

> **Honest scope:** v0.1 discovers and validates declarative modules. It does not execute plugin code, provide a sandbox, run a network API or claim production isolation.

## Quick start

Requires Rust stable (edition 2024).

    cargo build
    cargo test
    cargo run -- version
    cargo run -- doctor
    cargo run -- --config atlas.example.toml module list
    cargo run -- --config atlas.example.toml module plan

Copy atlas.example.toml to atlas.toml to configure the modules directory.

## Repository map

- src/manifest.rs — module package contract and validation.
- src/registry.rs — local module discovery.
- src/resolver.rs — dependency compatibility and startup ordering.
- src/policy.rs — explicit capability authorization.
- src/runtime.rs — lifecycle state model (no code execution).
- src/event.rs — bounded in-memory event queue.
- src/config.rs — validated TOML configuration.
- src/main.rs — ATLAS CLI.
- modules/ — declarative example modules.
- docs/ — architecture and security boundaries.

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Security model](docs/SECURITY.md)
- [Roadmap](ROADMAP.md)

## License

Apache-2.0. See LICENSE.
