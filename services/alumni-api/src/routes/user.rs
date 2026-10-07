//! Web routes: the user pages go to `UserController`.

use actix_web::web;

use crate::controllers::user_controller as users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(users::index))
            .route(web::post().to(users::create)),
    )
    .service(
        web::resource("/users/{id}")
            .route(web::get().to(users::show))
            .route(web::post().to(users::update)),
    )
    .service(web::resource("/users/{id}/edit").route(web::get().to(users::edit)))
    .service(web::resource("/users/{id}/delete").route(web::post().to(users::delete)));
}
