//! `ApiAlumniController`: JSON alumni CRUD under `/api/v1/alumni`.
//! Routes are defined in `routes::api_alumni`.

use actix_web::{HttpResponse, web};

use crate::AppState;
use crate::error::ApiError;
use crate::models::{Alumni, CreateAlumni, UpdateAlumni};

/// Creates an alumni record from the validated request body.
#[utoipa::path(
    post,
    path = "/api/v1/alumni",
    tag = "ApiAlumniController",
    request_body = CreateAlumni,
    responses(
        (status = 201, description = "Alumni created", body = Alumni),
        (status = 400, description = "Invalid fields or duplicate email"),
    ),
)]
pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateAlumni>,
) -> Result<HttpResponse, ApiError> {
    let alumni = state.alumni.create(&body).await?;
    Ok(HttpResponse::Created().json(alumni))
}

/// Lists all alumni, ordered by id.
#[utoipa::path(
    get,
    path = "/api/v1/alumni",
    tag = "ApiAlumniController",
    responses((status = 200, description = "All alumni", body = [Alumni])),
)]
pub async fn list(state: web::Data<AppState>) -> Result<web::Json<Vec<Alumni>>, ApiError> {
    Ok(web::Json(state.alumni.list().await?))
}

/// Returns one alumni record by id.
#[utoipa::path(
    get,
    path = "/api/v1/alumni/{id}",
    tag = "ApiAlumniController",
    params(("id" = u64, Path, description = "Alumni id")),
    responses(
        (status = 200, description = "The alumni record", body = Alumni),
        (status = 404, description = "No alumni record with that id"),
    ),
)]
pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<web::Json<Alumni>, ApiError> {
    Ok(web::Json(state.alumni.get(path.into_inner()).await?))
}

/// Replaces every field of an alumni record (404 if the id does not exist).
#[utoipa::path(
    put,
    path = "/api/v1/alumni/{id}",
    tag = "ApiAlumniController",
    request_body = CreateAlumni,
    params(("id" = u64, Path, description = "Alumni id")),
    responses(
        (status = 200, description = "Alumni replaced", body = Alumni),
        (status = 400, description = "Invalid fields or duplicate email"),
        (status = 404, description = "No alumni record with that id"),
    ),
)]
pub async fn replace(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<CreateAlumni>,
) -> Result<web::Json<Alumni>, ApiError> {
    Ok(web::Json(
        state.alumni.replace(path.into_inner(), &body).await?,
    ))
}

/// Applies a partial update to an alumni record (only provided fields change).
#[utoipa::path(
    patch,
    path = "/api/v1/alumni/{id}",
    tag = "ApiAlumniController",
    request_body = UpdateAlumni,
    params(("id" = u64, Path, description = "Alumni id")),
    responses(
        (status = 200, description = "Alumni updated", body = Alumni),
        (status = 400, description = "Invalid fields or duplicate email"),
        (status = 404, description = "No alumni record with that id"),
    ),
)]
pub async fn update(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<UpdateAlumni>,
) -> Result<web::Json<Alumni>, ApiError> {
    Ok(web::Json(
        state
            .alumni
            .update(path.into_inner(), body.into_inner())
            .await?,
    ))
}

/// Deletes an alumni record by id (400 if announcements reference it).
#[utoipa::path(
    delete,
    path = "/api/v1/alumni/{id}",
    tag = "ApiAlumniController",
    params(("id" = u64, Path, description = "Alumni id")),
    responses(
        (status = 204, description = "Alumni deleted"),
        (status = 400, description = "Alumni has announcements"),
        (status = 404, description = "No alumni record with that id"),
    ),
)]
pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    state.alumni.delete(path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
