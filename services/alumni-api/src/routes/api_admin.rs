//! API routes: admin endpoints under `/api/v1/admin` go to `ApiAdminController`.

use actix_web::web;

use crate::controllers::api_admin_controller as admin;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/v1/admin/users").route(web::get().to(admin::list_users)))
        .service(
            web::resource("/api/v1/admin/users/{id}").route(web::patch().to(admin::update_user)),
        )
        .service(
            web::resource("/api/v1/admin/users/{id}/verify")
                .route(web::post().to(admin::verify_user)),
        )
        .service(
            web::resource("/api/v1/admin/users/{id}/revoke-sessions")
                .route(web::post().to(admin::revoke_sessions)),
        );
}
