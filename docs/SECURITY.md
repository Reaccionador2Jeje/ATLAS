# Security model

- Unknown manifest fields are rejected.
- Module identifiers are constrained to lowercase ASCII segments.
- Entry points must be relative and may not traverse parent directories.
- Discovery is declarative: discovered module code is never executed.
- Capabilities are deny-by-default.
- Dependency versions use semantic-version requirements.
- The event queue has a fixed capacity.

These controls validate metadata and plans; they do not constitute a sandbox. Do not install untrusted module code or execute entrypoints until an isolated executor and package-signing design are implemented and reviewed.
