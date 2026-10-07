//! `ApiUserController`: versioned user routes under `/api/v1/users`. Register inside the `/api/v1` scope.

use actix_web::{HttpResponse, web};

use crate::error::ApiError;
use crate::models::{CreateUser, UpdateUser, User};
use crate::user_store::UserStore;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(list))
            .route(web::post().to(create)),
    )
    .service(
        web::resource("/users/{id}")
            .route(web::get().to(get))
            .route(web::put().to(replace))
            .route(web::patch().to(update))
            .route(web::delete().to(delete)),
    );
}

/// Creates a user from the validated request body.
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "ApiUserController",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 400, description = "Invalid name or email"),
    ),
)]
pub async fn create(
    store: web::Data<UserStore>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    let user = store.create(&body)?;
    Ok(HttpResponse::Created().json(user))
}

/// Lists all users, ordered by id.
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "ApiUserController",
    responses((status = 200, description = "All users", body = [User])),
)]
pub async fn list(store: web::Data<UserStore>) -> Result<web::Json<Vec<User>>, ApiError> {
    Ok(web::Json(store.list()?))
}

/// Returns one user by id.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "ApiUserController",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "The user", body = User),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn get(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<web::Json<User>, ApiError> {
    Ok(web::Json(store.get(path.into_inner())?))
}

/// Replaces every field of a user (404 if the id does not exist).
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "ApiUserController",
    request_body = CreateUser,
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "User replaced", body = User),
        (status = 400, description = "Invalid name or email"),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn replace(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
    body: web::Json<CreateUser>,
) -> Result<web::Json<User>, ApiError> {
    Ok(web::Json(store.replace(path.into_inner(), &body)?))
}

/// Applies a partial update to a user (only provided fields change).
#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}",
    tag = "ApiUserController",
    request_body = UpdateUser,
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 400, description = "Invalid name or email"),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn update(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
    body: web::Json<UpdateUser>,
) -> Result<web::Json<User>, ApiError> {
    Ok(web::Json(
        store.update(path.into_inner(), body.into_inner())?,
    ))
}

/// Deletes a user by id.
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "ApiUserController",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn delete(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    store.delete(path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}
