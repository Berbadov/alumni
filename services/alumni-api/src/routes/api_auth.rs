//! API routes: auth endpoints under `/api/v1/auth` go to `ApiAuthController`.

use actix_web::web;

use crate::controllers::api_auth_controller as auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/v1/auth/register").route(web::post().to(auth::register)))
        .service(web::resource("/api/v1/auth/login").route(web::post().to(auth::login)))
        .service(web::resource("/api/v1/auth/logout").route(web::post().to(auth::logout)))
        .service(
            web::resource("/api/v1/auth/me")
                .route(web::get().to(auth::me))
                .route(web::patch().to(auth::update_me)),
        );
}
