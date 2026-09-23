# Done

Completed work log, newest first.

## 2026-09-23 — shadcn/ui adoption
- `frontend`: Tailwind CSS v4 via `@tailwindcss/vite`; shadcn/ui (new-york style, neutral
  base color, CSS variables, `components.json` for the CLI).
- Base components in `src/components/ui/`: button, card, input, table
  (`npx shadcn add ...`). Runtime deps: `cn`, `radix-ui`, `class-variance-authority`,
  `lucide-react`.
- `@/*` path alias wired in `vite.config.ts` + `tsconfig.json`; theme in `src/index.css`.
- `App` renders the list in a shadcn `Card` + `Table`; inline styles removed.
- Verified: `npm run build` (tsc + vite build) passes. Docker image build not run here
  (daemon not available); `docker compose build frontend` covers it.

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
