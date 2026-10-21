//! API routes: job application CRUD under `/api/v1/job-applications`,
//! plus `/api/v1/job-postings/{id}/applications` for one posting.

use actix_web::web;

use crate::controllers::api_job_application_controller as applications;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/v1/job-applications")
            .route(web::get().to(applications::list))
            .route(web::post().to(applications::create)),
    )
    .service(
        web::resource("/api/v1/job-applications/{id}")
            .route(web::get().to(applications::get))
            .route(web::put().to(applications::replace))
            .route(web::patch().to(applications::update))
            .route(web::delete().to(applications::delete)),
    )
    .service(
        web::resource("/api/v1/job-postings/{id}/applications")
            .route(web::get().to(applications::list_for_posting)),
    );
}
