# Done

Completed work log, newest first.

## 2026-09-16 — MVP bootstrap
- Mono-repo scaffold: `services/alumni-api/`, `frontend/`, `docs/`.
- `alumni-api`: actix-web 4 service, edition 2024, rust-version 1.85, pedantic clippy lints.
  - `GET /api/v1/alumni` returns seeded alumni from MongoDB.
  - `AppState { collection }` shared via `web::Data`; `thiserror` `ApiError` + `ResponseError`.
  - Graceful shutdown via `ctrl_c` + 5s timeout; env-driven config with defaults.
- `frontend`: React + Vite + TypeScript; `App` fetches `/api/v1/alumni`, renders `AlumniList`.
  - Vite dev proxy forwards `/api` to `http://alumni-api:8080`.
- `docker-compose.yml`: `mongo` (volume), `mongo-seed` (one-shot `mongoimport`), `alumni-api`, `frontend`.
- Architecture diagrams: `docs/architecture/system-context.mmd`, `container.mmd`, `read-flow.mmd`.
- Docs: `backlog.md`, `done.md` (this), `trade-offs.md`.
