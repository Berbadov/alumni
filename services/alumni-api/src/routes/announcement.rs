//! Web routes: the announcement pages go to `AnnouncementController`.

use actix_web::web;

use crate::controllers::announcement_controller as announcements;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/announcements")
            .route(web::get().to(announcements::index))
            .route(web::post().to(announcements::create)),
    )
    .service(
        web::resource("/announcements/{id}")
            .route(web::get().to(announcements::show))
            .route(web::post().to(announcements::update)),
    )
    .service(
        web::resource("/announcements/{id}/edit").route(web::get().to(announcements::edit)),
    )
    .service(
        web::resource("/announcements/{id}/delete")
            .route(web::post().to(announcements::delete)),
    );
}
