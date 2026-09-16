# Trade-offs

Decisions and compromises, with rationale.

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
