//! `ApiJobPostingController`: JSON job posting CRUD under `/api/v1/job-postings`.
//! Routes are defined in `routes::api_job_posting`.

use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::AppState;
use crate::error::ApiError;
use crate::models::{CreateJobPosting, JobPosting, UpdateJobPosting};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListJobPostings {
    /// Filter by `is_active`. Omit it to list all postings.
    pub active: Option<bool>,
}

/// Creates a job posting from the validated request body.
#[utoipa::path(
    post,
    path = "/api/v1/job-postings",
    tag = "ApiJobPostingController",
    request_body = CreateJobPosting,
    responses(
        (status = 201, description = "Job posting created", body = JobPosting),
        (status = 400, description = "Invalid field, bad apply URL, or unknown author"),
    ),
)]
pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateJobPosting>,
) -> Result<HttpResponse, ApiError> {
    let posting = state.job_postings.create(&body).await?;
    Ok(HttpResponse::Created().json(posting))
}

/// Lists job postings, ordered by id. Use `?active=true` to hide inactive rows.
#[utoipa::path(
    get,
    path = "/api/v1/job-postings",
    tag = "ApiJobPostingController",
    params(ListJobPostings),
    responses((status = 200, description = "Job postings", body = [JobPosting])),
)]
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<ListJobPostings>,
) -> Result<web::Json<Vec<JobPosting>>, ApiError> {
    Ok(web::Json(state.job_postings.list(query.active).await?))
}

/// Returns one job posting by id.
#[utoipa::path(
    get,
    path = "/api/v1/job-postings/{id}",
    tag = "ApiJobPostingController",
    params(("id" = u64, Path, description = "Job posting id")),
    responses(
        (status = 200, description = "The job posting", body = JobPosting),
        (status = 404, description = "No job posting with that id"),
    ),
)]
pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<web::Json<JobPosting>, ApiError> {
    Ok(web::Json(state.job_postings.get(path.into_inner()).await?))
}

/// Replaces every field of a job posting (404 if the id does not exist).
#[utoipa::path(
    put,
    path = "/api/v1/job-postings/{id}",
    tag = "ApiJobPostingController",
    request_body = CreateJobPosting,
    params(("id" = u64, Path, description = "Job posting id")),
    responses(
        (status = 200, description = "Job posting replaced", body = JobPosting),
        (status = 400, description = "Invalid field, bad apply URL, or unknown author"),
        (status = 404, description = "No job posting with that id"),
    ),
)]
pub async fn replace(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<CreateJobPosting>,
) -> Result<web::Json<JobPosting>, ApiError> {
    Ok(web::Json(
        state
            .job_postings
            .replace(path.into_inner(), &body)
            .await?,
    ))
}

/// Applies a partial update to a job posting (only provided fields change).
#[utoipa::path(
    patch,
    path = "/api/v1/job-postings/{id}",
    tag = "ApiJobPostingController",
    request_body = UpdateJobPosting,
    params(("id" = u64, Path, description = "Job posting id")),
    responses(
        (status = 200, description = "Job posting updated", body = JobPosting),
        (status = 400, description = "Invalid field, bad apply URL, or unknown author"),
        (status = 404, description = "No job posting with that id"),
    ),
)]
pub async fn update(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<UpdateJobPosting>,
) -> Result<web::Json<JobPosting>, ApiError> {
    Ok(web::Json(
        state
            .job_postings
            .update(path.into_inner(), body.into_inner())
            .await?,
    ))
}

/// Deletes a job posting by id.
#[utoipa::path(
    delete,
    path = "/api/v1/job-postings/{id}",
    tag = "ApiJobPostingController",
    params(("id" = u64, Path, description = "Job posting id")),
    responses(
        (status = 204, description = "Job posting deleted"),
        (status = 404, description = "No job posting with that id"),
    ),
)]
pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    state.job_postings.delete(path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
