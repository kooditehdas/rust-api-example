# rust-api-example

A Rust HTTP API (axum) built with the [Kooditehdas factory](https://github.com/kooditehdas/factory).

## Develop

```bash
cargo run                  # listens on $PORT (default 8080)
curl localhost:8080/health
```

Checks: `cargo fmt --all --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, `cargo build --locked`. The toolchain version is pinned in `rust-toolchain.toml`.

## Working with the factory

1. Open an issue with the **Feature** template.
2. A maintainer adds `ai-ready` (implement now) or `ai-plan` (plan first, then add `ai-approved`).
3. The factory opens a PR from an `ai/issue-<n>-…` branch, CI runs, and the AI reviewer comments once.
4. Ask for changes with a PR comment starting `@factory`, e.g. `@factory fix review findings 1 and 3`. Ask for a fresh AI review with `@factory review`.
5. A human approves and merges.

Agent rules live in [AGENTS.md](AGENTS.md); factory settings in [factory.yaml](factory.yaml).
