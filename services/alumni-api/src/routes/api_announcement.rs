//! API routes: announcement CRUD under `/api/v1/announcements` goes to
//! `ApiAnnouncementController`. `/api/announcements` is an unversioned alias.

use actix_web::web;

use crate::controllers::api_announcement_controller as announcements;

const BASES: [&str; 2] = ["/api/v1/announcements", "/api/announcements"];

pub fn configure(cfg: &mut web::ServiceConfig) {
    for base in BASES {
        cfg.service(
            web::resource(base)
                .route(web::get().to(announcements::list))
                .route(web::post().to(announcements::create)),
        )
        .service(
            web::resource(format!("{base}/{{id}}"))
                .route(web::get().to(announcements::get))
                .route(web::put().to(announcements::replace))
                .route(web::patch().to(announcements::update))
                .route(web::delete().to(announcements::delete)),
        );
    }
}
