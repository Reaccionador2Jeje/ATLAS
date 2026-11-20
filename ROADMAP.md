# ATLAS roadmap

Roadmap entries describe planned work, not shipped capabilities.

## v0.1 — Core foundation
- [x] Rust core library and CLI.
- [x] Strict declarative module manifest.
- [x] Module discovery and duplicate-ID rejection.
- [x] Semantic-version dependency validation and deterministic startup planning.
- [x] Deny-by-default capability policy.
- [x] Lifecycle state coordinator.
- [x] Bounded event queue.
- [x] CI for formatting, Clippy, tests and release build.

## v0.2 — Service contracts
- [ ] Versioned local control API.
- [ ] Persistent configuration and structured diagnostics.
- [ ] Package checksums and audit log schema.

## v0.3 — Safe execution
- [ ] Isolated worker process protocol.
- [ ] Resource limits and cancellation.
- [ ] Filesystem and network sandbox profiles.
- [ ] Signed module packages and trust-store management.

## v0.4 — Ecosystem integrations
- [ ] SDKs and module templates.
- [ ] AtDev identity and service adapters.
- [ ] Storage, data and infrastructure provider interfaces.
- [ ] Extension catalog and reproducible builds.

## v1.0 — Stable platform
- [ ] Compatibility policy and migration tooling.
- [ ] Multi-platform installers.
- [ ] Operator documentation and threat-model review.
- [ ] Release automation and long-term support policy.
