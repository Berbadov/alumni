# Backlog

Planned work, prioritized top to bottom. Move items to `done.md` when complete
and record the rationale in `trade-offs.md` if the choice is non-obvious.

## Next
- [ ] **Alumni CRUD** — `POST`, `PUT`, `DELETE` for `/api/v1/alumni`; input validation; frontend form via shadcn `Button`/`Input`.
- [ ] **Schema hardening** — required vs optional fields, email format, unique email index.

## Later
- [ ] **Search / filter** — query by graduation year, degree, name; pagination + sorting.
- [ ] **Alumni profiles** — individual detail page + `GET /api/v1/alumni/{id}`.
- [ ] **Auth & sessions** — class admin login; restrict write endpoints.
- [ ] **Events** — alumni events model + endpoints.
- [ ] **Notifications service** — split out as `services/notifications/` microservice (email/reminders).
- [ ] **CI pipeline** — lint, clippy pedantic, build, docker compose smoke test.
- [ ] **Production frontend serving** — build static assets, serve via alumni-api or a separate static container.
