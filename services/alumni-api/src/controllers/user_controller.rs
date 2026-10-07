//! `UserController`: HTML pages for users. Routes are defined in `routes::user`.
//!
//! HTML forms only send GET and POST, so update and delete are POST routes.

use actix_web::http::StatusCode;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{HttpResponse, web};
use maud::Markup;

use crate::error::ApiError;
use crate::models::CreateUser;
use crate::user_store::{UserError, UserStore};
use crate::views::users::{self, Draft};

fn html(status: StatusCode, page: Markup) -> HttpResponse {
    HttpResponse::build(status)
        .content_type(ContentType::html())
        .body(page.into_string())
}

fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

/// Turns a missing user into an HTML 404 page instead of a JSON error.
fn respond(result: Result<HttpResponse, UserError>) -> Result<HttpResponse, ApiError> {
    match result {
        Err(UserError::NotFound) => Ok(html(StatusCode::NOT_FOUND, users::not_found())),
        other => Ok(other?),
    }
}

/// Lists the users and shows the create form.
#[utoipa::path(
    get,
    path = "/users",
    tag = "UserController",
    responses(
        (status = 200, description = "HTML page with the user list and a create form",
            content_type = "text/html", body = String),
    ),
)]
pub async fn index(store: web::Data<UserStore>) -> Result<HttpResponse, ApiError> {
    respond(
        store
            .list()
            .map(|all| html(StatusCode::OK, users::index(&all, None))),
    )
}

/// Creates a user from the form, then redirects to `/users`.
#[utoipa::path(
    post,
    path = "/users",
    tag = "UserController",
    request_body(content = CreateUser, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 303, description = "User created; redirects to /users"),
        (status = 400, description = "Invalid name or email; HTML page with the error",
            content_type = "text/html", body = String),
    ),
)]
pub async fn create(
    store: web::Data<UserStore>,
    form: web::Form<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    respond(match store.create(&form) {
        Ok(_) => Ok(see_other("/users")),
        Err(UserError::Invalid(error)) => store.list().map(|all| {
            let draft = Draft {
                input: &form,
                error,
            };
            html(StatusCode::BAD_REQUEST, users::index(&all, Some(&draft)))
        }),
        Err(err) => Err(err),
    })
}

/// Shows one user.
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "UserController",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "HTML page with the user", content_type = "text/html", body = String),
        (status = 404, description = "HTML page: no user with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn show(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    respond(
        store
            .get(path.into_inner())
            .map(|user| html(StatusCode::OK, users::show(&user))),
    )
}

/// Shows the edit form for one user.
#[utoipa::path(
    get,
    path = "/users/{id}/edit",
    tag = "UserController",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "HTML page with the edit form", content_type = "text/html", body = String),
        (status = 404, description = "HTML page: no user with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn edit(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    respond(
        store
            .get(path.into_inner())
            .map(|user| html(StatusCode::OK, users::edit(&user, None))),
    )
}

/// Replaces the name and email of a user from the form, then redirects to the user page.
#[utoipa::path(
    post,
    path = "/users/{id}",
    tag = "UserController",
    params(("id" = u64, Path, description = "User id")),
    request_body(content = CreateUser, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 303, description = "User updated; redirects to /users/{id}"),
        (status = 400, description = "Invalid name or email; HTML edit page with the error",
            content_type = "text/html", body = String),
        (status = 404, description = "HTML page: no user with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn update(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
    form: web::Form<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    respond(match store.replace(id, &form) {
        Ok(_) => Ok(see_other(&format!("/users/{id}"))),
        Err(UserError::Invalid(error)) => store.get(id).map(|user| {
            let draft = Draft {
                input: &form,
                error,
            };
            html(StatusCode::BAD_REQUEST, users::edit(&user, Some(&draft)))
        }),
        Err(err) => Err(err),
    })
}

/// Deletes a user, then redirects to `/users`.
#[utoipa::path(
    post,
    path = "/users/{id}/delete",
    tag = "UserController",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 303, description = "User deleted; redirects to /users"),
        (status = 404, description = "HTML page: no user with that id", content_type = "text/html", body = String),
    ),
)]
pub async fn delete(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    respond(
        store
            .delete(path.into_inner())
            .map(|()| see_other("/users")),
    )
}
