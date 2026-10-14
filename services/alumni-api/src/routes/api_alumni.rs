//! API routes: alumni CRUD under `/api/v1/alumni` goes to `ApiAlumniController`.

use actix_web::web;

use crate::controllers::api_alumni_controller as alumni;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/v1/alumni")
            .route(web::get().to(alumni::list))
            .route(web::post().to(alumni::create)),
    )
    .service(
        web::resource("/api/v1/alumni/{id}")
            .route(web::get().to(alumni::get))
            .route(web::put().to(alumni::replace))
            .route(web::patch().to(alumni::update))
            .route(web::delete().to(alumni::delete)),
    );
}
