# AGENTS.md — dgr

Agent-facing repo brief. Read this before editing.

## Rules that bind

- **Specs first (LeanSpec).** A feat or fix creates or updates a LeanSpec spec under `specs/`
  before code, and the PR names that spec (for example `SPEC-0001` or `REQ-0001-02`). The
  registry, lifecycle and EARS conventions are in `specs/README.md`.
- **Gates:** `cargo test --release --locked`, as CI runs it. Commits are signed. Changes land
  through PRs into `main`; no direct pushes.
