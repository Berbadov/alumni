# Done

Completed work log, newest first.

## 2026-10-21 — Job applications and Leptos frontend
- `docs/ddd.md`: new `job_applications` collection with `job_posting_id`,
  `alumni_id`, `cover_letter`, `status`, `created_at`, and `updated_at`.
  Statuses: `submitted`, `reviewed`, `accepted`, `rejected`; default
  `submitted`. One application per alumni and posting.
- `services/alumni-api/migrations/002_add_job_applications.js`: idempotent
  collection, unique compound index, `alumni_id` and `status` indexes, and
  counter initialization.
- Seed: 3 job postings and 3 applications; compose resets all four counters.
- `JobApplicationStore` and `ApiJobApplicationController` serve JSON CRUD under
  `/api/v1/job-applications` plus `GET /api/v1/job-postings/{id}/applications`.
  Delete of a posting or alumni with applications fails with 400.
- Replaced React with Leptos CSR. Pages: home, alumni, jobs, job detail/apply,
  applications with status update, about, and 404. Plain CSS keeps the theme.
  Trunk builds the WASM app; nginx serves it and proxies `/api/`.
- Tests: 34 pass (1 model, 3 route). Clippy passes with `-D warnings`.
- Verified in Docker: seed counts, application API, SPA routes, and gateway.

## 2026-10-21 — Job postings: schema, migration, and JSON CRUD
- `docs/ddd.md`: new `job_postings` collection. Fields: `title`, `company`,
  `description`, `location`, `employment_type`, `apply_url`, `author_id`,
  `is_active`, `created_at`, `updated_at`. `author_id` links to `alumni._id`
  and blocks alumni delete. Counters and the ER diagram include the collection.
- `services/alumni-api/migrations/001_add_job_postings.js`: idempotent
  `mongosh` migration. It creates the collection, the `author_id` and
  `is_active` indexes, and the `job_postings` counter. The counter uses
  `$setOnInsert` and starts from the largest existing `_id` to prevent
  duplicate ids.
- `alumni-api`: `JobPosting`, `CreateJobPosting`, and `UpdateJobPosting` in
  `models.rs`, with trim, non-empty, URL-prefix, and author checks.
  `JobPostingStore` runs the MongoDB CRUD on the `job_postings` counter.
- `ApiJobPostingController` serves JSON CRUD under `/api/v1/job-postings`.
  `is_active` defaults to `true`. `?active=true|false` filters the list.
  Unknown `author_id` fails with 400. Startup creates both indexes.
  Swagger lists the six operations under `ApiJobPostingController`.
- Delete of an alumni record with job postings fails with 400.
- Tests: 30 pass (2 new model tests, 4 new route tests). Clippy passes with
  `-D warnings`. Verified in Docker (`rust:1.98`) against `mongo:7`.
- Open: frontend job posting page.

## 2026-10-14 — MongoDB migration per `docs/ddd.md`
- `users` merges into `alumni`. `UserStore`, `UserController`,
  `ApiUserController`, the `/users` pages, and the `/api/v1/users` plus
  `/api/users` routes are gone. The gateway `/users` proxy is gone too.
- `alumni-api`: `AlumniStore` and `AnnouncementStore` run on MongoDB with
  atomic `counters` (`findOneAndUpdate` with `$inc`, upsert). Ids start at 1.
  Seed rows are `1/2/3` with BSON dates and int types.
- `ApiAlumniController`: JSON CRUD under `/api/v1/alumni` only. Duplicate
  email fails with 400. Delete of alumni with announcements fails with 400.
- `Announcement.author_id` replaces the free-text `author`. Unknown author
  fails with 400. HTML forms take `author_id`.
- Data rules stay in the Model (`models.rs` pure functions plus unit tests).
  Timestamps are RFC3339 strings in JSON and BSON dates in storage.
- Startup creates the unique `alumni.email` index and the
  `announcements.author_id` index. Compose starts the API after `mongo-seed`
  finishes: the first boot built the index while `--drop` ran and MongoDB
  aborted the build.
- Tests: 24 pass. `cargo test` needs MongoDB on `localhost:27017` (or sets
  `MONGO_URI`). Each test uses its own database and drops it after. Clippy
  shows zero warnings.
- Verified in Docker: seed import plus counters, full alumni and announcement
  CRUD, block delete, old user routes return 404, gateway on :80 serves the
  frontend with numeric alumni ids.

## 2026-10-07 — Announcements: model, controllers, routes, web CRUD
- `alumni-api`: new `Announcement` model (`id`, `title`, `body`, `author`) with
  `CreateAnnouncement` / `UpdateAnnouncement` DTOs in `models.rs`, and an in-memory
  `AnnouncementStore` (`src/announcement_store.rs`) mirroring `UserStore`: sequential ids,
  trim-on-save, non-empty validation, `AnnouncementError` (`Invalid`, `NotFound`, `Poisoned`)
  mapped to `ApiError` via `From`.
- Two controllers, same shape as the user pair:
  - `AnnouncementController` — HTML pages under `/announcements` (index + create form,
    show, edit, update, delete), form posts with 303 redirects, 400 re-render on invalid
    input, HTML 404 page on a missing id. Views in `src/views/announcements.rs` (maud).
  - `ApiAnnouncementController` — JSON CRUD under `/api/v1/announcements` (alias
    `/api/announcements`): POST 201, GET list/get, PUT replace, PATCH partial, DELETE 204.
- Routes: `routes/announcement.rs` and `routes/api_announcement.rs`, wired into
  `routes::configure`; store registered in `main.rs`. Swagger lists all 12 operations with
  the new schemas and tags.
- Gateway: `frontend/nginx.conf` proxies `/announcements` and `/announcements/...` to the API.
- Tests: 42 pass (6 announcement model tests, 6 JSON route tests per prefix, 9 page tests;
  user tests unchanged). Clippy shows only the 2 pre-existing `io_other_error` warnings in
  `main.rs`.
- Design spec: `docs/superpowers/specs/2026-10-07-announcements-design.md`.
- Verified natively with `cargo test` + `cargo clippy` (the Docker engine was down);
  a Docker build and a gateway check on port 80 are still open.

## 2026-10-07 — User routes, view layer and web CRUD
- `UserController` now serves HTML pages. Routes: `GET /users` (list and create form),
  `POST /users` (create), `GET /users/{id}` (show), `GET /users/{id}/edit` (edit form),
  `POST /users/{id}` (update), `POST /users/{id}/delete` (delete). HTML forms send only GET
  and POST. Success redirects with 303. Invalid input shows the page again with status 400,
  the error and the typed values. A missing user shows an HTML 404 page.
- `ApiUserController` keeps the JSON CRUD under `/api/v1/users`. The unversioned `/api/users`
  stays as an alias of the same handlers.
- New `src/routes/`: `user.rs` and `api_user.rs` define the paths. Controllers hold only
  handlers. `main.rs` calls `routes::configure` before the `/api/v1` scope.
- New `src/views/` (maud): `layout.rs` and `users.rs` (index, show, edit, not found).
- Swagger: all 12 user operations are listed. Tags `UserController` (HTML pages) and
  `ApiUserController` (JSON) have descriptions. The HTML routes show `text/html` and form
  request bodies.
- Gateway: `frontend/nginx.conf` proxies `/users` and `/users/...` to the API.
- Tests: 21 pass (6 model tests, 6 JSON route tests, 9 page tests).
  Clippy shows only the 2 existing `io_other_error` warnings in `main.rs`.
- Verified on a native run of the API (the Docker engine was down): all 6 page routes
  (200, 303, 400, 404), the JSON routes, `/api/openapi.json` and the Swagger UI. A Docker
  build and a check through the gateway on port 80 are still open.

## 2026-10-07 — UserController and ApiUserController
- `alumni-api`: new `src/controllers/` with two controllers, each holding the six CRUD
  route functions (`create`, `list`, `get`, `replace`, `update`, `delete`) and a `configure`:
  - `user_controller`: `/api/users` (the existing unversioned routes).
  - `api_user_controller`: `/api/v1/users` (versioned, registered in the `/api/v1` scope).
- Both call the shared `UserStore`. `src/users.rs` is removed.
- Swagger lists both route sets, grouped by the tags `UserController` and `ApiUserController`.
- Tests: 7 route tests (lifecycle, ordered list and invalid input for each prefix, plus one
  that checks both prefixes share one store) and 6 model tests. `cargo test` passes (13);
  clippy shows only the 2 existing `io_other_error` warnings in `main.rs`.
- Verified in Docker: full CRUD on `/api/v1/users`, cross-prefix reads (create on one prefix,
  read on the other), 400 and 404 errors, nginx gateway, `/api/openapi.json` lists all paths.

## 2026-10-07 — User model extracted from the handlers
- `alumni-api`: new `src/user_store.rs` with the in-memory `UserStore` model (no database,
  no actix types): `create`, `get`, `list`, `replace`, `update`, `delete`, plus the data
  rules (`validate_name`, `validate_email`, trim and lowercase).
- Model errors use `UserError` (`Invalid`, `NotFound`, `Poisoned`). `error.rs` maps it to
  `ApiError` with `From<UserError>`, so the Model does not know about HTTP.
- `src/users.rs` now holds only the route functions. Each one calls one `UserStore` method.
  Routes, status codes and OpenAPI output are unchanged.
- Tests: 6 unit tests on `UserStore` (no actix) plus the 3 existing route tests.
  `cargo test` passes (9); clippy shows only the 2 existing `io_other_error` warnings in
  `main.rs`.

## 2026-09-30 — `/api/health`, in-memory user CRUD, Swagger UI
- `alumni-api`: `GET /api/health` returns `{"status":"ok"}`.
- `alumni-api`: user CRUD in `src/users.rs` with a `UserStore` (`AtomicU64` ids +
  `Mutex<BTreeMap<u64, User>>`) — no database, data resets on restart:
  - `POST /api/users` → 201 (name trimmed, email lowercased), `GET /api/users`,
    `GET /api/users/{id}`, `PUT /api/users/{id}` (full replace), `PATCH /api/users/{id}`
    (partial), `DELETE /api/users/{id}` → 204.
  - Validation: non-empty name, email with non-empty local part and domain → 400 on failure.
- Swagger: `utoipa` 5 + `utoipa-swagger-ui` 9; OpenAPI at `/api/openapi.json`,
  UI at `/api/swagger/` (`GET /api/swagger` 302-redirects to it). Paths use `/api/...`
  per request, not `/api/v1` (see `trade-offs.md`).
- `error.rs`: new `BadRequest(String)` (400) and `Internal(String)` (500) `ApiError` variants.
- `Dockerfile` now copies `Cargo.lock` into the dependency layer (lock was committed for
  reproducible builds but never copied).
- Tests: 3 actix-web integration tests in `src/users.rs` (lifecycle, ordered list,
  invalid input) — `cargo test` passes; clippy clean (2 pre-existing `io_other_error`
  warnings in `main.rs` unchanged).
- Verified in Docker (`docker compose up --build`): health 200, POST 201, PATCH/PUT 200,
  DELETE 204 then 404, bad email 400, Swagger UI 200, old `/api/v1` routes unaffected;
  all also reachable through the nginx gateway on :80 (`/api/users`, `/api/swagger/`).
- Tooling: Postman 12.30.3 installed on the host via winget.

## 2026-09-23 — `/about` page with Prufrock poem
- `frontend`: new `src/pages/About.tsx` — T.S. Eliot's "The Love Song of J. Alfred Prufrock"
  (1915, public domain) excerpts in a shadcn `Card` (`CardTitle`/`CardDescription`/`CardContent`),
  poem stanzas in `font-serif italic` on the theme's `card-foreground` token, with
  `whitespace-pre-line` to keep the line breaks.
- Routing: `about` route in `App.tsx`, "About" `NavLink` in `Layout.tsx` header (active state
  styled like Home/Alumni). Works at `/about` and `/about/` (router treats the trailing slash
  as optional).
- Verified: `npm run build` (tsc + vite) passes; `docker compose up --build frontend` —
  `/about` → `200 text/html`, and the served bundle (hash `index-Dh8X5DxM.js`, same as the
  host build) contains the poem text. Diagrams unchanged — they don't enumerate SPA routes.

## 2026-09-23 — `GET /api/v1/sum/{num1}/{num2}` bare JSON sum
- `alumni-api`: `handlers::sum` takes `web::Path<(f64, f64)>` and returns `web::Json(num1 + num2)`
  — bare JSON number, no wrapper. Floats after an i64 first cut (sums like `1.5 + 2` were 404s);
  plain `+` since f64 overflow is `inf`, not a panic.
- Gateway: `frontend/nginx.conf` adds prefix `location /sum/` proxying to `/api/v1/sum/`,
  same pattern as `/hello/`.
- Verified in Docker: `5.0 [200 application/json]` from `127.0.0.1/sum/2/3`,
  `127.0.0.1/api/v1/sum/2/3`, and `127.0.0.1:8080/api/v1/sum/2/3`; decimals work
  (`/sum/1.5/2` → `3.5`); negatives (`/sum/-5/3` → `-2.0`); non-numeric input (`/sum/2/abc`)
  and missing segments (`/sum/2`) → 404 (actix `Path` parse errors).

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
