//! `ApiAnnouncementController`: JSON announcement CRUD under `/api/v1/announcements`.
//! Routes are defined in `routes::api_announcement`.

use actix_web::{HttpResponse, web};

use crate::AppState;
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
    ),
)]
pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateAnnouncement>,
) -> Result<HttpResponse, ApiError> {
    let announcement = state.announcements.create(&body).await?;
    Ok(HttpResponse::Created().json(announcement))
}

/// Lists all announcements, ordered by id.
#[utoipa::path(
    get,
    path = "/api/v1/announcements",
    tag = "ApiAnnouncementController",
    responses((status = 200, description = "All announcements", body = [Announcement])),
)]
pub async fn list(state: web::Data<AppState>) -> Result<web::Json<Vec<Announcement>>, ApiError> {
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
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<web::Json<Announcement>, ApiError> {
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
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn replace(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<CreateAnnouncement>,
) -> Result<web::Json<Announcement>, ApiError> {
    Ok(web::Json(
        state
            .announcements
            .replace(path.into_inner(), &body)
            .await?,
    ))
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
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn update(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<UpdateAnnouncement>,
) -> Result<web::Json<Announcement>, ApiError> {
    Ok(web::Json(
        state
            .announcements
            .update(path.into_inner(), body.into_inner())
            .await?,
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
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    state.announcements.delete(path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
