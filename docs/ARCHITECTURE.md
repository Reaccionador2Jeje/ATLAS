# ATLAS architecture — foundation

ATLAS is an ecosystem, not a single application. This repository begins with shared contracts for future products and services.

## Layers
1. Core contracts: module manifests, stable identifiers, API versioning and shared errors.
2. Registry: discovers declarative module packages and rejects duplicate or malformed IDs.
3. Dependency planner: validates semantic-version requirements and produces deterministic dependency-first order.
4. Policy: denies capabilities by default; installations explicitly grant requested capabilities.
5. Lifecycle coordinator: tracks ready, running and stopped states. It is not a process executor in v0.1.
6. Event queue: bounded in-memory event transport.
7. CLI: diagnostics and inspection.

## Module package
Each module lives in its own directory and contains atlas.module.toml. The manifest declares identity, version, supported API, dependencies, requested capabilities and an optional relative entrypoint. Entrypoint is metadata only in this release.

## Trust boundary
ATLAS v0.1 does not dynamically load libraries, run scripts, spawn module processes, expose a network API or claim sandbox isolation. Discovery parses data only. Capability checks are authorization planning, not an OS security boundary. A future executor requires process isolation, explicit filesystem/network policy, signed packages and auditable lifecycle transitions before third-party code is enabled.

## Compatibility
The module API version is separate from the ATLAS product version. Breaking manifest changes require an API version change and migration notes.
