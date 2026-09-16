# AGENTS.md — Alumni Tracking System

Project rules for agents (and humans) working in this repo. Rules below condense
the `actionbook/rust-skills` and `community-actix-web` skill guides, then add
project-specific overrides that win on conflict within this repo.

## Rust (from `actionbook/rust-skills`)

- Edition 2024; `rust-version = "1.85"`.
- `clippy`: `all = "warn"`, `pedantic = "warn"`; `#![warn(unsafe_code)]`.
- Naming: `snake_case` for functions/vars, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for consts.
- Line width 100 chars.
- Prefer `?` over `unwrap()`/`expect()` in non-test code.
- Every `unsafe` block carries a `// SAFETY:` comment explaining the invariant.
- Error handling: return `Result` with context; no panics in handler paths.

## actix-web (from `community-actix-web`)

- Typed extractors: `web::Path`, `web::Query`, `web::Json`, `web::Data`.
- Shared state via `web::Data<AppState>`.
- Middleware via `.wrap(...)`; keep `Logger` on the app.
- Errors: `thiserror` enum implementing `actix_web::error::ResponseError`.
- Routes scoped under `/api/v1`: `web::scope("/api/v1")`.
- Server: configure `workers`, bind `BIND_ADDR`, graceful shutdown via `ctrl_c` + timeout.

## Project overrides (win on conflict in this repo)

- **Simple, goal-achieving code.** Minimal comments — only for non-obvious behavior, never for deliberation.
- **Everything runs in Docker.** Docs assume `docker compose`; no host-run steps unless explicitly noted.
- **Docs are first-class.** Every feature touches:
  - `docs/backlog.md` (planned work, prioritized)
  - `docs/done.md` (completed work log)
  - `docs/trade-offs.md` (decisions and compromises)
  - Architecture changes must update `docs/architecture/*.mmd`.
- **Diagrams in Mermaid** (`.mmd`), text-based so agents edit them and GitHub renders them inline.
- **REST under `/api/v1`.** One backend service `alumni-api` for now; new microservices land under `services/<name>/` with their own `Dockerfile` and a `docker-compose` entry.
- **Mono-repo layout:** `services/` for backends, `frontend/` for the web client.
