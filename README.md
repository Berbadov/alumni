# Alumni Tracking System

Alumni tracking app for the Web Development class of fall 2026-2027.

## Stack

- **Backend:** Rust + actix-web 4, MongoDB (official driver). Service: `alumni-api`.
- **Frontend:** React + Vite + TypeScript, Tailwind CSS + shadcn/ui.
- **Database:** MongoDB 7.
- **Runtime:** everything in Docker via `docker compose`.

## Layout

```
alumni/
  services/alumni-api/   # Rust actix-web backend
  frontend/               # React + Vite client
  docs/                   # backlog, done log, trade-offs, architecture diagrams
  docker-compose.yml
  AGENTS.md
```

## Run

All services run in Docker — no host toolchain required.

```bash
docker compose up --build
```

Then:

- Frontend: http://localhost (nginx serves the built SPA on :80; Vite dev server on :5173 is for host-side development only)
- API: http://localhost:8080/api/v1/alumni

The first `up` seeds MongoDB with sample alumni via the `mongo-seed` container.

## Routes

SPA pages: `/` (home), `/alumni` (list), `/about` (placeholder with a Prufrock poem);
unknown paths fall back to the SPA router's 404 page.

API (all under `/api/v1`, reachable via the API on :8080 or the gateway on :80):

| Route | Returns |
| --- | --- |
| `GET /api/v1/alumni` | seeded alumni from MongoDB |
| `GET /api/v1/hello` | `"hello, world"` |
| `GET /api/v1/hello/{variable}` | `"hello, {variable}!"` |
| `GET /api/v1/sum/{num1}/{num2}` | `num1 + num2` as a bare JSON number (f64) |

Gateway conveniences on :80 (proxied to the API, see `docs/trade-offs.md`):
`/hello`, `/hello/{variable}`, `/sum/{num1}/{num2}` — the same responses without the
`/api/v1` prefix.

## Documentation

- `docs/architecture/` — Mermaid diagrams (system context, containers, read flow) + how to view them.
- `docs/backlog.md` — planned work, prioritized.
- `docs/done.md` — completed work log.
- `docs/trade-offs.md` — decisions and compromises.

See `AGENTS.md` for the conventions agents and contributors must follow.
