# Agent instructions

These rules apply to every AI agent working in this repository (implementer and reviewer alike) and to humans.

## Project

An HTTP JSON API in Rust with [axum](https://docs.rs/axum) on tokio. The toolchain version is pinned in `rust-toolchain.toml`.

- `src/main.rs` binds the port and serves with graceful shutdown. Keep it small; no business logic.
- `src/lib.rs` builds the router: `app()` registers every route and then calls `with_error_handling`, which must stay last because axum layers only wrap routes added before them.
- One module per resource (for example `src/health.rs`), with `pub(crate)` handlers registered in `app()`.
- `src/error.rs` holds `ApiError`, the only error response type.
- Integration tests live in `tests/` and send requests through the router with `tower::ServiceExt::oneshot`, never a real port or network.

## Commands

| Purpose | Command                                               |
| ------- | ----------------------------------------------------- |
| Fetch   | `cargo fetch --locked`                                |
| Format  | `cargo fmt --all`                                     |
| Lint    | `cargo clippy --all-targets --locked -- -D warnings`  |
| Test    | `cargo test --locked`                                 |
| Build   | `cargo build --locked`                                |

Format check, lint, test and build must all pass before work is considered done. Clippy runs with the `pedantic` group (see `[lints]` in `Cargo.toml`), and warnings fail the build.

## Conventions

- Responses are JSON. Handlers return `Json<T>` for success and `ApiError` (or `Result<Json<T>, ApiError>`) for failures, giving `{ "error": "<message>" }` with a matching status code.
- Validate input at the handler boundary with typed extractors (`Json<T>`, `Path<T>`, `Query<T>`) and serde; reject invalid values with a 400 `ApiError`.
- No `unwrap()` or `expect()` in `src/` outside of startup in `main.rs`; propagate errors with `?`. Tests may use `expect` with a message.
- `unsafe` is forbidden.
- Every new route gets an integration test for the success path and at least one failure path.
- Prefer the crates already in `Cargo.toml`. A new dependency needs a clear reason stated in the PR; add it with `cargo add` so `Cargo.lock` stays in sync.

## Constraints

- Never commit secrets, tokens or real credentials. Configuration comes from environment variables documented in `.env.example`.
- Never weaken, skip (`#[ignore]`) or delete tests to make a check pass, and never silence lints with `#[allow(...)]` without an explaining comment.
- Do not change unrelated code, formatting or files.
- Do not edit `.github/`, `factory.yaml`, `.factory/` or `rust-toolchain.toml` unless the task explicitly asks for it.

## Definition of done

1. The acceptance criteria in the issue are met.
2. Tests cover the change.
3. Format check, lint, test and build pass locally.
4. The change is summarized with any remaining risks or follow-ups.
