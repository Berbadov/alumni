//! `ApiJobApplicationController`: JSON job application CRUD under
//! `/api/v1/job-applications`, plus a nested list for one posting.

use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::AppState;
use crate::error::ApiError;
use crate::models::{
    CreateJobApplication, JobApplication, JobApplicationFilter, UpdateJobApplication,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListJobApplications {
    /// Filter by job posting id.
    pub job_posting_id: Option<u64>,
    /// Filter by alumni id.
    pub alumni_id: Option<u64>,
    /// Filter by status: submitted, reviewed, accepted, or rejected.
    pub status: Option<String>,
}

fn filter(query: &ListJobApplications) -> JobApplicationFilter {
    JobApplicationFilter {
        job_posting_id: query.job_posting_id,
        alumni_id: query.alumni_id,
        status: query.status.clone(),
    }
}

/// Creates a job application from the validated request body.
#[utoipa::path(
    post,
    path = "/api/v1/job-applications",
    tag = "ApiJobApplicationController",
    request_body = CreateJobApplication,
    responses(
        (status = 201, description = "Application created", body = JobApplication),
        (status = 400, description = "Unknown posting/alumni, bad status, or duplicate application"),
    ),
)]
pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateJobApplication>,
) -> Result<HttpResponse, ApiError> {
    let application = state.job_applications.create(&body).await?;
    Ok(HttpResponse::Created().json(application))
}

/// Lists applications, ordered by id. All filters are optional.
#[utoipa::path(
    get,
    path = "/api/v1/job-applications",
    tag = "ApiJobApplicationController",
    params(ListJobApplications),
    responses((status = 200, description = "Job applications", body = [JobApplication])),
)]
pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<ListJobApplications>,
) -> Result<web::Json<Vec<JobApplication>>, ApiError> {
    Ok(web::Json(
        state.job_applications.list(&filter(&query)).await?,
    ))
}

/// Lists applications for one job posting.
#[utoipa::path(
    get,
    path = "/api/v1/job-postings/{id}/applications",
    tag = "ApiJobApplicationController",
    params(("id" = u64, Path, description = "Job posting id")),
    responses((status = 200, description = "Applications for the posting", body = [JobApplication])),
)]
pub async fn list_for_posting(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<web::Json<Vec<JobApplication>>, ApiError> {
    let filter = JobApplicationFilter {
        job_posting_id: Some(path.into_inner()),
        ..JobApplicationFilter::default()
    };
    Ok(web::Json(state.job_applications.list(&filter).await?))
}

/// Returns one application by id.
#[utoipa::path(
    get,
    path = "/api/v1/job-applications/{id}",
    tag = "ApiJobApplicationController",
    params(("id" = u64, Path, description = "Application id")),
    responses(
        (status = 200, description = "The application", body = JobApplication),
        (status = 404, description = "No application with that id"),
    ),
)]
pub async fn get(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<web::Json<JobApplication>, ApiError> {
    Ok(web::Json(
        state.job_applications.get(path.into_inner()).await?,
    ))
}

/// Replaces every field of an application (404 if the id does not exist).
#[utoipa::path(
    put,
    path = "/api/v1/job-applications/{id}",
    tag = "ApiJobApplicationController",
    request_body = CreateJobApplication,
    params(("id" = u64, Path, description = "Application id")),
    responses(
        (status = 200, description = "Application replaced", body = JobApplication),
        (status = 400, description = "Unknown posting/alumni, bad status, or duplicate application"),
        (status = 404, description = "No application with that id"),
    ),
)]
pub async fn replace(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<CreateJobApplication>,
) -> Result<web::Json<JobApplication>, ApiError> {
    Ok(web::Json(
        state
            .job_applications
            .replace(path.into_inner(), &body)
            .await?,
    ))
}

/// Applies a partial update to an application (only provided fields change).
#[utoipa::path(
    patch,
    path = "/api/v1/job-applications/{id}",
    tag = "ApiJobApplicationController",
    request_body = UpdateJobApplication,
    params(("id" = u64, Path, description = "Application id")),
    responses(
        (status = 200, description = "Application updated", body = JobApplication),
        (status = 400, description = "Unknown posting/alumni, bad status, or duplicate application"),
        (status = 404, description = "No application with that id"),
    ),
)]
pub async fn update(
    state: web::Data<AppState>,
    path: web::Path<u64>,
    body: web::Json<UpdateJobApplication>,
) -> Result<web::Json<JobApplication>, ApiError> {
    Ok(web::Json(
        state
            .job_applications
            .update(path.into_inner(), body.into_inner())
            .await?,
    ))
}

/// Deletes an application by id.
#[utoipa::path(
    delete,
    path = "/api/v1/job-applications/{id}",
    tag = "ApiJobApplicationController",
    params(("id" = u64, Path, description = "Application id")),
    responses(
        (status = 204, description = "Application deleted"),
        (status = 404, description = "No application with that id"),
    ),
)]
pub async fn delete(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    state.job_applications.delete(path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
