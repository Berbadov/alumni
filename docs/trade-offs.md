# Trade-offs

Decisions and compromises, with rationale.

## Bare smoke-test routes at the gateway (`/hello`, `/sum`) vs `/api/v1` only
`GET /hello` on port 80 is a requested convenience: a one-URL smoke check without the API
prefix. It deviates from the "REST under `/api/v1`" rule, so the deviation lives at the edge —
nginx's exact-match `location = /hello` rewrites to the API's canonical `/api/v1/hello`, and the
API itself keeps everything scoped. Exact match (not a prefix location) so it cannot shadow
future SPA routes like `/hello-world`. The parameterized routes (`/hello/{variable}`,
`/sum/{num1}/{num2}`) get bare-gateway twins via prefix locations with a trailing slash
(`/hello/`, `/sum/`) — the trailing slash keeps `/hello-world`-style SPA routes out of their
reach, so only SPA paths literally under those prefixes (none today) would be shadowed.

## alumni-api build: rust 1.98 image + committed Cargo.lock vs rust-version 1.85
The API's first real Docker build failed on both toolchain and code (see `done.md`): a fresh
dependency resolve pulls `yoke-derive 0.8.3`, which does not compile on rustc 1.85, and the app
code targeted the mongodb 2.x API. Bumped the build image to `rust:1.98` and committed
`Cargo.lock` so image builds are reproducible instead of re-resolving on every build.
Caveat: `Cargo.toml` keeps `rust-version = "1.85"` per the rust-skills guide, but the locked
dependency tree now effectively requires the newer toolchain — a host rustc pinned to 1.85
will not build the service. Revisit the declared MSRV when the guide's floor moves.

## Frontend serving: nginx inside the frontend image vs separate proxy service
Chose nginx in the frontend image (multi-stage build: node compiles `dist/`, nginx serves it).
A separate proxy container would decouple routing config from the app image and allow
proxying future microservices without rebuilding the frontend, but for one backend it adds
a container and a config surface for no gain. The `/api/` proxy rule lives in
`frontend/nginx.conf` and can move out if services multiply.

## Dropping the Vite dev server from compose
Compose now runs the production build on :80 instead of `vite dev` on :5173. The dev
container gave hot reload in Docker, but it served unminified dev assets as "the deployment"
and masked build breakage — `npm run dev` on the host (with the Vite `/api` proxy) covers
iteration, and compose now exercises the artifact that actually ships. Cost: no in-Docker
hot reload; API is still reachable directly on :8080 for host-side dev.

## UI kit: shadcn/ui vs ObsidianUI vs hand-rolled CSS
Chose shadcn/ui. ObsidianUI (evaluated 2026-09-23) is an effects library — cursor trails,
WebGL backgrounds, text animations — with no forms, tables, or dialogs, so it cannot carry
a CRUD-heavy tracking app. shadcn/ui ships app components, copies MIT source into the repo
(no runtime dependency lock-in), and adds components on demand via the shadcn CLI. Costs:
Tailwind in the build chain and a larger bundle (~58 kB gz JS vs a plain HTML list).
ObsidianUI remains an option later for landing-page flourishes via the same CLI.

## Frontend: React + Vite vs htmx
Chose React + Vite. Larger bundle and a client-side stack, but it is the most common
agent-friendly setup: typed TS, large ecosystem, clear component model. htmx would be
smaller and server-driven, but ties the UI to server-rendered HTML and is harder to grow
into a richer client. For a class project meant to grow, the ecosystem wins.

## MVP scope: read-only list vs full CRUD
Shipped only `GET /api/v1/alumni`. Read-only deliberately minimizes surface area while
exercising the full Mongo + Docker + actix stack (connect, query, serialize, serve,
container networking, seed). CRUD is the top backlog item — the foundation it lands on
is already proven.

## Database: MongoDB vs SQL
MongoDB. Alumni records are flexible documents (varying degrees, optional fields) and
the official Rust driver integrates cleanly with serde/bson. SQL would add migrations
and a stricter schema earlier than the MVP needs. Schema discipline is tracked in backlog.

## Single service vs premature microservices
One backend service `alumni-api`. Microservices are scaffolded for (layout is
`services/<name>/` with per-service Dockerfile and a compose entry), but none created yet.
Splitting now would add operational cost without a real boundary; `notifications` is the
first candidate when the need is concrete.

## Diagrams: Mermaid vs external tools
Mermaid `.mmd` text diagrams. Slightly less polished than draw.io/PlantUML, but text-based:
agents can edit them in-tree, diff cleanly, and GitHub renders them inline. Diagram-first
documentation is the user priority, and Mermaid best fits that within this workflow.

## Rust edition 2024 + rust-version 1.85
Required by the rust-skills guide. Edition 2024 is newer; pins `rust-version = "1.85"` in
`Cargo.toml` so toolchain expectations are explicit. The host here lacks cargo, so the Rust
build was not run in this environment — see verification notes in the implementation summary.
