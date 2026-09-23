# Done

Completed work log, newest first.

## 2026-09-23 — `GET /api/v1/sum/{num1}/{num2}` bare JSON sum
- `alumni-api`: `handlers::sum` takes `web::Path<(i64, i64)>` and returns
  `web::Json(num1.saturating_add(num2))` — bare JSON integer, no wrapper; saturating so an
  overflow cannot panic in the handler path.
- Gateway: `frontend/nginx.conf` adds prefix `location /sum/` proxying to `/api/v1/sum/`,
  same pattern as `/hello/`.
- Verified in Docker: `5 [200 application/json]` from `127.0.0.1/sum/2/3`,
  `127.0.0.1/api/v1/sum/2/3`, and `127.0.0.1:8080/api/v1/sum/2/3`; negatives work
  (`/sum/-5/3` → `-2`); overflow saturates (`i64::MIN + -1` → `i64::MIN`); non-numeric
  input (`/sum/2/abc`) and missing segments (`/sum/2`) → 404 (actix `Path` parse errors).

## 2026-09-23 — `GET /api/v1/hello/{variable}` parameterized greeting
- `alumni-api`: `handlers::hello_name` takes `web::Path<String>` and returns
  `web::Json(format!("hello, {variable}!"))`, registered alongside `hello` in the `/api/v1` scope.
- Gateway: `frontend/nginx.conf` adds prefix `location /hello/` proxying to the API's
  `/api/v1/hello/`, so `http://127.0.0.1/hello/beraat` works on port 80 too; the exact-match
  `location = /hello` is unchanged.
- Verified in Docker: `"hello, beraat!" [200 application/json]` from `127.0.0.1/hello/beraat`,
  `127.0.0.1/api/v1/hello/beraat`, and `127.0.0.1:8080/api/v1/hello/beraat`; URL-encoded
  segments decode (`world%20123` → `"hello, world 123!"`); plain `/hello` and SPA fallback
  (`/hello-world` → index.html) unaffected.

## 2026-09-23 — `GET /hello` / `GET /api/v1/hello` smoke-test endpoint
- `alumni-api`: `handlers::hello` returns `web::Json("hello, world")` (`200 application/json`,
  bare JSON string, no wrapper), registered in the existing `/api/v1` scope.
- Gateway: `frontend/nginx.conf` adds an exact-match `location = /hello` proxying to the API's
  `/api/v1/hello`, so `http://127.0.0.1/hello` returns it on the gateway port too (see
  `trade-offs.md` for the convention deviation).
- Verified in Docker: `"hello, world" [200 application/json]` from `127.0.0.1/hello`,
  `127.0.0.1/api/v1/hello`, and `127.0.0.1:8080/api/v1/hello`; SPA routes (`/`, `/alumni`,
  unknown paths) unaffected.

## 2026-09-23 — alumni-api Docker build fix (latent, surfaced by first compose build)
The API image had never been built (no cargo/docker on the authoring host), and its first
real build failed:
- `rust:1.85` + lockless resolve picks `yoke-derive 0.8.3`, whose `str::from_utf8` does not
  compile on 1.85 — build image bumped to `rust:1.98`.
- App code was written against the mongodb 2.x API and an actix signature that never existed:
  `find(None, None)` → `find(doc! {})` (mongodb 3.6 single-filter signature);
  `shutdown_timeout(Duration::from_secs(5))` → `shutdown_timeout(5)` (takes `u64` seconds);
  `server.handle()` on `HttpServer` → `run()` first, then `handle()` on the returned `Server`,
  `server.await` to completion.
- Committed `services/alumni-api/Cargo.lock` (binary crate — reproducible image builds) and
  `services/alumni-api/.gitignore` (`/target`).
- Verified: `cargo build --release` clean in `rust:1.98` container (one pre-existing
  `dead_code` warning: unused `ApiError::NotFound`, kept for the planned detail endpoint).

## 2026-09-23 — Routing, theme, and production serving on :80
- `frontend`: client-side routing via `react-router-dom` v7 — `BrowserRouter` in `main.tsx`,
  `Routes` in `App.tsx`: `/` → Home, `/alumni` → Alumni (list moved from `App`), `*` → NotFound.
- `src/components/Layout.tsx`: header (nav Home/Alumni + `ThemeToggle`) and footer around an
  `Outlet`; `src/components/ThemeToggle.tsx` toggles `.dark` on `<html>`, persists to
  `localStorage('alumni-theme')`; inline pre-paint script in `index.html` applies the class
  before first paint (defaults to `prefers-color-scheme`).
- Theme tokens in `src/index.css` rewritten onto the green→blue palette
  (`d9ed92 … 184e77`): light background `#d9ed92` / dark `#184e77`, primary `#34a0a4` / `#76c893`.
- Production serving: multi-stage `frontend/Dockerfile` (node:24-alpine build → nginx:alpine),
  `frontend/nginx.conf` serves `dist/` on :80 with `try_files` SPA fallback and `/api/` proxied
  to `alumni-api:8080`; `docker-compose.yml` publishes `80:80` (was dev server on :5173).
  Vite dev server remains for host-side development.
- Architecture diagrams updated: `container.mmd` (nginx + static SPA on :80, /api proxied),
  `read-flow.mmd` (frontend is nginx + SPA). Backlog item "production frontend serving" closed.
- Verified: `npm run build` passes; `docker compose up --build` then `curl` checks —
  `/alumni` 200, `/` 200, `/api/v1/alumni` 200 (proxied), unknown paths fall back to the SPA.

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
