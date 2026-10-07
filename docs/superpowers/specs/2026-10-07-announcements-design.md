# Announcements Feature — Design

Date: 2026-10-07
Branch: `user-routes-views`

## Goal

Add announcement management to `alumni-api`, mirroring the existing User feature:
an in-memory model, a JSON API controller, an HTML controller with management
pages, and routes for both.

## Scope

In scope:

- `Announcement` model + `AnnouncementStore` (in-memory, same shape as `UserStore`)
- `ApiAnnouncementController` — JSON CRUD under `/api/v1/announcements`
- `AnnouncementController` — HTML management pages under `/announcements`
- Routes for both controllers, including the unversioned `/api/announcements` alias
- Tests mirroring the existing user route tests
- Doc updates: `backlog.md` (React page as later work), `done.md`, `trade-offs.md`,
  `container.mmd` (it enumerates API surfaces)

Out of scope (recorded in backlog):

- React frontend page for announcements
- Persistence (data resets on restart, like users)

## Data model

`models.rs`:

```rust
pub struct Announcement {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub author: String,
}

pub struct CreateAnnouncement { title, body, author: String }
pub struct UpdateAnnouncement { title, body, author: Option<String> } // serde default
```

All `Serialize`/`Deserialize`, `ToSchema` for utoipa, matching `User`'s derives.

## Store

`announcement_store.rs`, same structure as `user_store.rs`:

- `AnnouncementStore { next_id: AtomicU64, announcements: Mutex<BTreeMap<u64, Announcement>> }`
- `AnnouncementError`: `Invalid(&'static str)`, `NotFound`, `Poisoned` (thiserror, PartialEq/Eq)
- Methods: `create`, `get`, `list`, `replace`, `update`, `delete`
- Validation: title, body, author all non-empty after trimming; values trimmed on save
- Ids start at 1

## Controllers

`controllers/api_announcement_controller.rs` — `ApiAnnouncementController`:

- `create` (POST `/api/v1/announcements`, 201)
- `list` (GET, 200, ordered by id)
- `get` (GET `/{id}`, 200/404)
- `replace` (PUT `/{id}`, 200/400/404)
- `update` (PATCH `/{id}`, partial, 200/400/404)
- `delete` (DELETE `/{id}`, 204/404)

Each with utoipa `#[utoipa::path]` annotations, tagged `ApiAnnouncementController`,
registered in the OpenAPI doc in `main.rs`.

`controllers/announcement_controller.rs` — `AnnouncementController`:

- `index`: GET `/announcements` — table of announcements + create form
- `create`: POST `/announcements` — form post, 303 redirect to `/announcements`;
  invalid input re-renders the page with the error and submitted values
- `show`: GET `/announcements/{id}` — details + edit link
- `edit`: GET `/announcements/{id}/edit` — edit form
- `update`: POST `/announcements/{id}` — 303 redirect to `/announcements/{id}`
- `delete`: POST `/announcements/{id}/delete` — 303 redirect to `/announcements`
- Missing id renders an HTML 404 page

## Routes

- `routes/api_announcement.rs`: JSON CRUD at `/api/v1/announcements` and the
  `/api/announcements` alias, both backed by the same handlers
- `routes/announcement.rs`: HTML pages at `/announcements` (+ sub-paths)
- Both wired into `routes::configure`
- `web::Data<AnnouncementStore>` registered in `main.rs` alongside `UserStore`

## Views

`views/announcements.rs` (Maud), same structure as `views/users.rs`:

- `index`, `show`, `edit`, `Draft`/error rendering, all HTML-escaped by Maud

## Error handling

`ApiError` (in `error.rs`) gains a `From<AnnouncementError>` mapping:
`Invalid` → 400, `NotFound` → 404, `Poisoned` → 500. The HTML controller converts
`AnnouncementError` the same way `UserController` handles `UserError` (400 with
re-rendered form for invalid input, 404 page for missing ids).

## Testing

Mirrors of the existing test groups in `routes/mod.rs`:

- API lifecycle test at `/api/v1/announcements` and `/api/announcements`
- List ordering, invalid input rejection (400), missing id (404)
- HTML index escaping, form create + redirect, invalid form input error re-render
- Show/edit pages, form update + redirect, form delete + redirect
- HTML 404 page for missing ids

Verification: `cargo test` and `cargo clippy` inside Docker.

## Docs

- `docs/backlog.md`: add "Announcements React page" under Later; move the
  announcements backend work to `done.md` when complete
- `docs/trade-offs.md`: in-memory store reset on restart (consistent with users)
- `docs/architecture/container.mmd`: add `/announcements: HTML pages` and
  `/api/v1/announcements: JSON CRUD` to the alumni-api node
