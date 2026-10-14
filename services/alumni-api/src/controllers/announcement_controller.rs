//! `AnnouncementController`: HTML pages for announcements. Routes are defined in
//! `routes::announcement`.
//!
//! HTML forms only send GET and POST, so update and delete are POST routes.

use actix_web::http::StatusCode;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{HttpResponse, web};
use maud::Markup;

use crate::AppState;
use crate::error::ApiError;
use crate::models::{CreateAnnouncement, ModelError};
use crate::views::announcements::{self, Draft};

fn html(status: StatusCode, page: Markup) -> HttpResponse {
    HttpResponse::build(status)
        .content_type(ContentType::html())
        .body(page.into_string())
}

fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

/// Turns a missing announcement into an HTML 404 page instead of a JSON error.
fn respond(result: Result<HttpResponse, ModelError>) -> Result<HttpResponse, ApiError> {
    match result {
        Err(ModelError::NotFound) => Ok(html(StatusCode::NOT_FOUND, announcements::not_found())),
        other => Ok(other?),
    }
}

/// Lists the announcements and shows the create form.
#[utoipa::path(
    get,
    path = "/announcements",
    tag = "AnnouncementController",
    responses(
        (status = 200, description = "HTML page with the announcement list and a create form",
            content_type = "text/html", body = String),
    ),
)]
pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    respond(
        state
            .announcements
            .list()
            .await
            .map(|all| html(StatusCode::OK, announcements::index(&all, None))),
    )
}

/// Creates an announcement from the form, then redirects to `/announcements`.
#[utoipa::path(
    post,
    path = "/announcements",
    tag = "AnnouncementController",
    request_body(content = CreateAnnouncement, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 303, description = "Announcement created; redirects to /announcements"),
        (status = 400, description = "Invalid input; HTML page with the error",
            content_type = "text/html", body = String),
    ),
)]
pub async fn create(
    state: web::Data<AppState>,
    form: web::Form<CreateAnnouncement>,
) -> Result<HttpResponse, ApiError> {
    respond(match state.announcements.create(&form).await {
        Ok(_) => Ok(see_other("/announcements")),
        Err(ModelError::Invalid(error)) => state.announcements.list().await.map(|all| {
            let draft = Draft {
                input: &form,
                error,
            };
            html(StatusCode::BAD_REQUEST, announcements::index(&all, Some(&draft)))
        }),
        Err(err) => Err(err),
    })
}

/// Shows one announcement.
#[utoipa::path(
    get,
    path = "/announcements/{id}",
    tag = "AnnouncementController",
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 200, description = "HTML page with the announcement", content_type = "text/html", body = String),
        (status = 404, description = "HTML page: no announcement with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn show(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    respond(
        state
            .announcements
            .get(path.into_inner())
            .await
            .map(|announcement| html(StatusCode::OK, announcements::show(&announcement))),
    )
}

/// Shows the edit form for one announcement.
#[utoipa::path(
    get,
    path = "/announcements/{id}/edit",
    tag = "AnnouncementController",
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 200, description = "HTML page with the edit form", content_type = "text/html", body = String),
        (status = 404, description = "HTML page: no announcement with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn edit(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    respond(
        state
            .announcements
            .get(path.into_inner())
            .await
            .map(|announcement| html(StatusCode::OK, announcements::edit(&announcement, None))),
    )
}

/// Replaces the fields of an announcement from the form, then redirects to its page.
#[utoipa::path(
    post,
    path = "/announcements/{id}",
    tag = "AnnouncementController",
    params(("id" = u64, Path, description = "Announcement id")),
    request_body(content = CreateAnnouncement, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 303, description = "Announcement updated; redirects to /announcements/{id}"),
        (status = 400, description = "Invalid input; HTML edit page with the error",
            content_type = "text/html", body = String),
        (status = 404, description = "HTML page: no announcement with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn update(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    form: web::Form<CreateAnnouncement>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    respond(match state.announcements.replace(id, &form).await {
        Ok(_) => Ok(see_other(&format!("/announcements/{id}"))),
        Err(ModelError::Invalid(error)) => state.announcements.get(id).await.map(|announcement| {
            let draft = Draft {
                input: &form,
                error,
            };
            html(
                StatusCode::BAD_REQUEST,
                announcements::edit(&announcement, Some(&draft)),
            )
        }),
        Err(err) => Err(err),
    })
}

/// Deletes an announcement, then redirects to `/announcements`.
#[utoipa::path(
    post,
    path = "/announcements/{id}/delete",
    tag = "AnnouncementController",
    params(("id" = u64, Path, description = "Announcement id")),
    responses(
        (status = 303, description = "Announcement deleted; redirects to /announcements"),
        (status = 404, description = "HTML page: no announcement with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    respond(
        state
            .announcements
            .delete(path.into_inner())
            .await
            .map(|()| see_other("/announcements")),
    )
}
