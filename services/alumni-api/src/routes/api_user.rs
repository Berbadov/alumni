//! API routes: user CRUD under `/api/v1/users` goes to `ApiUserController`.
//! `/api/users` is an unversioned alias of the same handlers.

use actix_web::web;

use crate::controllers::api_user_controller as users;

const BASES: [&str; 2] = ["/api/v1/users", "/api/users"];

pub fn configure(cfg: &mut web::ServiceConfig) {
    for base in BASES {
        cfg.service(
            web::resource(base)
                .route(web::get().to(users::list))
                .route(web::post().to(users::create)),
        )
        .service(
            web::resource(format!("{base}/{{id}}"))
                .route(web::get().to(users::get))
                .route(web::put().to(users::replace))
                .route(web::patch().to(users::update))
                .route(web::delete().to(users::delete)),
        );
    }
}
