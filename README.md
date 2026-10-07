# Alumni Tracking System

This is an alumni tracking app for the Web Development class of fall 2026-2027.

## Writing style

Write the README and all files in `docs/` in Simplified Technical English (ASD-STE100).

- Use short sentences. Use the active voice.
- Use one term for one thing. Do not use synonyms.
- Use the imperative form for steps.
- Do not use idioms, contractions, or abbreviations that you do not define.

## Status

The backend is Rust and runs today. The frontend plan is a Rust WASM app built with Leptos.
The repository still contains the React frontend. It is temporary. The Leptos frontend replaces
it, and all `.ts` and `.tsx` files go away. The Stack, Layout and MVC sections describe the
planned frontend. The sections for the backend describe the code as it is today.

## Stack

- **Backend:** Rust, actix-web 4, MongoDB (official driver). Service: `alumni-api`.
- **Frontend (planned):** Rust compiled to WASM. Framework: Leptos in client-side rendering (CSR)
  mode. Build tool: Trunk. Styling: Tailwind CSS. nginx serves the built files.
- **Shared model (planned):** one Rust crate, `alumni-model`, used by the backend and the frontend.
- **Database:** MongoDB 7.
- **Runtime:** all parts run in Docker with `docker compose`.

## Layout

```
alumni/
  services/alumni-api/    # Rust actix-web backend
  frontend/               # Rust + Leptos WASM client (planned; React client until then)
  crates/alumni-model/    # Shared Model structs (planned)
  docs/                   # backlog, done log, trade-offs, architecture diagrams
  docker-compose.yml
  AGENTS.md
```

## MVC structure

The system uses the Model-View-Controller (MVC) pattern across the whole stack.

| Layer | Job | Where it lives |
| --- | --- | --- |
| Model | Holds the data, the data rules and the data storage. | `alumni-model` crate, `alumni-api` data code, MongoDB |
| View | Shows data to the user. | Leptos pages and components, nginx |
| Controller | Receives input, calls the Model and selects the result. | `alumni-api` handlers, frontend `controllers/` module |

```mermaid
flowchart LR
    User["User"] --> View
    subgraph Frontend["Frontend: Rust WASM"]
        View["View: pages, components"] --> FC["Controller: controllers/"]
    end
    FC -->|"JSON over /api"| BC
    subgraph Backend["Backend: alumni-api"]
        BC["Controller: handlers"] --> Model["Model: structs, stores"]
    end
    Model --> DB[("MongoDB")]
    Shared["alumni-model crate"] -.-> View
    Shared -.-> Model
```

### Request flow: list the alumni

```mermaid
sequenceDiagram
    participant V as View (alumni page)
    participant FC as Frontend controller
    participant BC as Backend handler
    participant M as Model (MongoDB)
    V->>FC: Page opens
    FC->>BC: GET /api/v1/alumni
    BC->>M: find all alumni
    M-->>BC: Alumni documents
    BC-->>FC: JSON array
    FC-->>V: Signal with alumni
    V->>V: Render the table
```

### Backend mapping (current code)

Docker Compose starts all layers. It is the shared wiring and is not part of one layer.

| Layer | Part | File or container |
| --- | --- | --- |
| Model | Data shapes `Alumni`, `User`, `CreateUser`, `UpdateUser` | `services/alumni-api/src/models.rs` |
| Model | MongoDB collection handle | `AppState` in `main.rs` |
| Model | In-memory user store `UserStore` with `create`, `get`, `list`, `replace`, `update`, `delete` | `services/alumni-api/src/user_store.rs` |
| Model | Data rules: `validate_name`, `validate_email` and the `UserError` type | `services/alumni-api/src/user_store.rs` |
| Model | Database settings and bind address | `services/alumni-api/src/config.rs` |
| Model | Sample data and import | `seed/alumni.json`, `mongo-seed` container |
| Model | Database | `mongo` container |
| Controller | Route functions for health, hello, sum and alumni | `services/alumni-api/src/handlers.rs` |
| Controller | `UserController`: user CRUD routes under `/api/users` | `services/alumni-api/src/controllers/user_controller.rs` |
| Controller | `ApiUserController`: user CRUD routes under `/api/v1/users` | `services/alumni-api/src/controllers/api_user_controller.rs` |
| Controller | Error type and HTTP status mapping `ApiError` | `services/alumni-api/src/error.rs` |
| Controller | Route registration, `/api/v1` scope, OpenAPI and Swagger UI | `services/alumni-api/src/main.rs` |
| View | JSON responses that the API returns | Serde derive on the Model structs |

### Frontend mapping (planned)

| Layer | Part | Planned location |
| --- | --- | --- |
| Model | `Alumni` and `User` structs | `crates/alumni-model/` |
| Controller | Functions and signals that call the API and hold the page state | `frontend/src/controllers/` |
| Controller | Route table | `frontend/src/app.rs` |
| View | Page components | `frontend/src/pages/` |
| View | Shared components (layout, theme toggle, table) | `frontend/src/components/` |
| View | Tailwind CSS styles | `frontend/` stylesheet |
| View | Static file server | `frontend` container (nginx) |

### Rules

- A View must not call the API. It must call a controller.
- A controller must not hold data rules. It must call the Model.
- The Model must not know about HTTP, routes or pages.
- The backend and the frontend must import `Alumni` and `User` from `alumni-model`. Do not copy them.

### Known gaps

- `list_alumni` in `handlers.rs` queries MongoDB directly. No separate Model function exists.
- Two data sources exist: MongoDB for alumni, and memory for users. The user data resets on restart.
  See `docs/trade-offs.md`.
- The `alumni-model` crate and the `crates/` folder are not in the repository yet. `AGENTS.md`
  must list the `crates/` folder when you add it.
- Docker builds for `alumni-api` and the frontend must use the repository root as build context
  to read the shared crate.

## Run

All services run in Docker. You do not need a host toolchain.

```bash
docker compose up --build
```

Then open these addresses:

- Frontend: http://localhost (nginx serves the built app on port 80)
- API: http://localhost:8080/api/v1/alumni

The first `up` command fills MongoDB with sample alumni. The `mongo-seed` container does this.

## Routes

The SPA has these pages: `/` (home), `/alumni` (list) and `/about` (placeholder with a Prufrock
poem). For an unknown path, the SPA router shows the 404 page.

The API has these routes under `/api/v1`. Use port 8080 for the API or port 80 for the gateway.

| Route | Returns |
| --- | --- |
| `GET /api/v1/alumni` | The alumni from MongoDB |
| `GET /api/v1/hello` | `"hello, world"` |
| `GET /api/v1/hello/{variable}` | `"hello, {variable}!"` |
| `GET /api/v1/sum/{num1}/{num2}` | `num1 + num2` as a bare JSON number (f64) |

The user CRUD and tool routes are under `/api`. The store is in memory and resets on restart.
See `docs/trade-offs.md`.

| Route | Returns |
| --- | --- |
| `GET /api/health` | `{"status": "ok"}` |
| `POST /api/users` | The new user (status 201). Fields: `name`, `email` |
| `GET /api/users` | All users, ordered by id |
| `GET /api/users/{id}` | One user, or status 404 |
| `PUT /api/users/{id}` | Full replacement, or status 404 |
| `PATCH /api/users/{id}` | Partial update, or status 404 |
| `DELETE /api/users/{id}` | Status 204, or status 404 |
| `GET /api/swagger` | Swagger UI (redirects to `/api/swagger/`) |

The Swagger UI shows all routes: http://localhost:8080/api/swagger. The gateway also serves it
at http://localhost/api/swagger.

The gateway on port 80 proxies these short routes to the API: `/hello`, `/hello/{variable}` and
`/sum/{num1}/{num2}`. They return the same responses as the routes without the `/api/v1` prefix.
See `docs/trade-offs.md`.

## Documentation

- `docs/architecture/` - Mermaid diagrams (system context, containers, read flow) and how to view them.
- `docs/backlog.md` - planned work, in priority order.
- `docs/done.md` - log of completed work.
- `docs/trade-offs.md` - decisions and compromises.

Read `AGENTS.md` for the rules that agents and contributors must obey.
