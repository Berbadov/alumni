# Alumni Tracking System

Alumni tracking app for the Web Development class of fall 2026-2027.

## Stack

- **Backend:** Rust + actix-web 4, MongoDB (official driver). Service: `alumni-api`.
- **Frontend:** React + Vite + TypeScript.
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

- Frontend: http://localhost:5173
- API: http://localhost:8080/api/v1/alumni

The first `up` seeds MongoDB with sample alumni via the `mongo-seed` container.

## Documentation

- `docs/architecture/` — Mermaid diagrams (system context, containers, read flow) + how to view them.
- `docs/backlog.md` — planned work, prioritized.
- `docs/done.md` — completed work log.
- `docs/trade-offs.md` — decisions and compromises.

See `AGENTS.md` for the conventions agents and contributors must follow.
