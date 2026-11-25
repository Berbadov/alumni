# Backlog

Planned work, prioritized top to bottom. Move items to `done.md` when complete
and record the rationale in `trade-offs.md` if the choice is non-obvious.

## Next
- [ ] **Alumni form** — frontend create/edit form via shadcn `Button`/`Input` talking to `/api/v1/alumni`.

## Later
- [ ] **About page content** — replace the Prufrock placeholder with real project/team info.
- [ ] **Search / filter** — query by graduation year, degree, name; pagination + sorting.
- [ ] **Alumni profiles** — individual detail page (API `GET /api/v1/alumni/{id}` exists).
- [ ] **Paper upload & SMS/email activation** — store uploaded government-paper documents and enable `REQUIRE_EMAIL_VERIFICATION` / `REQUIRE_SMS_VERIFICATION` with provider wiring.
- [ ] **Events** — alumni events model + endpoints.
- [ ] **Notifications service** — split out as `services/notifications/` microservice (email/reminders).
- [ ] **CI pipeline** — lint, clippy pedantic, build, docker compose smoke test.
