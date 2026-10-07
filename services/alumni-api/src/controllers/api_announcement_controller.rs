//! `ApiAnnouncementController`: JSON announcement CRUD under `/api/v1/announcements`.
//! Routes are defined in `routes::api_announcement`.

use actix_web::{HttpResponse, web};

use crate::announcement_store::AnnouncementStore;
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
        (status = 400, description = "Empty title, body or author"),
    ),
)]
pub async fn create(
    store: web::Data<AnnouncementStore>,
    body: web::Json<CreateAnnouncement>,
) -> Result<HttpResponse, ApiError> {
    let announcement = store.create(&body)?;
    Ok(HttpResponse::Created().json(announcement))
}

/// Lists all announcements, ordered by id.
#[utoipa::path(
    get,
    path = "/api/v1/announcements",
    tag = "ApiAnnouncementController",
    responses((status = 200, description = "All announcements", body = [Announcement])),
)]
pub async fn list(
    store: web::Data<AnnouncementStore>,
) -> Result<web::Json<Vec<Announcement>>, ApiError> {
    Ok(web::Json(store.list()?))
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
    store: web::Data<AnnouncementStore>,
    path: web::Path<u64>,
) -> Result<web::Json<Announcement>, ApiError> {
    Ok(web::Json(store.get(path.into_inner())?))
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
        (status = 400, description = "Empty title, body or author"),
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn replace(
    store: web::Data<AnnouncementStore>,
    path: web::Path<u64>,
    body: web::Json<CreateAnnouncement>,
) -> Result<web::Json<Announcement>, ApiError> {
    Ok(web::Json(store.replace(path.into_inner(), &body)?))
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
        (status = 400, description = "Empty title, body or author"),
        (status = 404, description = "No announcement with that id"),
    ),
)]
pub async fn update(
    store: web::Data<AnnouncementStore>,
    path: web::Path<u64>,
    body: web::Json<UpdateAnnouncement>,
) -> Result<web::Json<Announcement>, ApiError> {
    Ok(web::Json(store.update(path.into_inner(), body.into_inner())?))
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
    store: web::Data<AnnouncementStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    store.delete(path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}
