//! API routes: job posting CRUD under `/api/v1/job-postings` goes to
//! `ApiJobPostingController`.

use actix_web::web;

use crate::controllers::api_job_posting_controller as postings;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/v1/job-postings")
            .route(web::get().to(postings::list))
            .route(web::post().to(postings::create)),
    )
    .service(
        web::resource("/api/v1/job-postings/{id}")
            .route(web::get().to(postings::get))
            .route(web::put().to(postings::replace))
            .route(web::patch().to(postings::update))
            .route(web::delete().to(postings::delete)),
    );
}
