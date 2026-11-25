//! `ApiAnnouncementController`: JSON announcement CRUD under `/api/v1/announcements`.
//! Routes are defined in `routes::api_announcement`.

use actix_web::{HttpResponse, web};

use crate::AppState;
use crate::auth_middleware::AuthenticatedUser;
use crate::error::ApiError;
use crate::models::{Announcement, CreateAnnouncement, UpdateAnnouncement};

/// Creates an announcement from the validated request body.
#[utoipa::path(
    post,
    path = "/api/v1/announcements",
    tag = "ApiAnnouncementController",
    request_body = CreateAnnouncement,
    responses(
        (status = 201, description = "Announcement created", body = Announcement),
        (status = 400, description = "Empty title or body, or unknown author"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Insufficient permissions"),
    ),
)]
pub async fn create(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    body: web::Json<CreateAnnouncement>,
) -> Result<HttpResponse, ApiError> {
    actor.can_create_announcement()?;
    actor.can_act_as_author(body.author_id)?;
    let announcement = state.announcements.create(&body).await?;
    Ok(HttpResponse::Created().json(announcement))
}

/// Lists all announcements, ordered by id.
#[utoipa::path(
    get,
    path = "/api/v1/announcements",
    tag = "ApiAnnouncementController",
    responses(
        (status = 200, description = "All announcements", body = [Announcement]),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Account verification pending"),
    ),
)]
pub async fn list(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
) -> Result<web::Json<Vec<Announcement>>, ApiError> {
    actor.can_read_directory()?;
    Ok(web::Json(state.announcements.list().await?))
}

/// Returns one announcement by id.
#[utoipa::path(
    get,
    path = "/api/v1/announcements/{id}",
    tag = "ApiAnnouncementController",
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 200, description = "The announcement", body = Announcement),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Account verification pending"),
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn get(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
) -> Result<web::Json<Announcement>, ApiError> {
    actor.can_read_directory()?;
    Ok(web::Json(state.announcements.get(path.into_inner()).await?))
}

/// Replaces every field of an announcement (404 if the id does not exist).
#[utoipa::path(
    put,
    path = "/api/v1/announcements/{id}",
    tag = "ApiAnnouncementController",
    request_body = CreateAnnouncement,
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 200, description = "Announcement replaced", body = Announcement),
        (status = 400, description = "Empty title or body, or unknown author"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn replace(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
    body: web::Json<CreateAnnouncement>,
) -> Result<web::Json<Announcement>, ApiError> {
    let id = path.into_inner();
    let existing = state.announcements.get(id).await?;
    actor.can_modify_announcement(existing.author_id)?;
    actor.can_act_as_author(body.author_id)?;
    Ok(web::Json(state.announcements.replace(id, &body).await?))
}

/// Applies a partial update to an announcement (only provided fields change).
#[utoipa::path(
    patch,
    path = "/api/v1/announcements/{id}",
    tag = "ApiAnnouncementController",
    request_body = UpdateAnnouncement,
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 200, description = "Announcement updated", body = Announcement),
        (status = 400, description = "Empty title or body, or unknown author"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn update(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
    body: web::Json<UpdateAnnouncement>,
) -> Result<web::Json<Announcement>, ApiError> {
    let id = path.into_inner();
    let existing = state.announcements.get(id).await?;
    actor.can_modify_announcement(existing.author_id)?;
    if let Some(author_id) = body.author_id {
        actor.can_act_as_author(author_id)?;
    }
    Ok(web::Json(
        state.announcements.update(id, body.into_inner()).await?,
    ))
}

/// Deletes an announcement by id.
#[utoipa::path(
    delete,
    path = "/api/v1/announcements/{id}",
    tag = "ApiAnnouncementController",
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 204, description = "Announcement deleted"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn delete(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let existing = state.announcements.get(id).await?;
    actor.can_modify_announcement(existing.author_id)?;
    state.announcements.delete(id).await?;
    Ok(HttpResponse::NoContent().finish())
}
