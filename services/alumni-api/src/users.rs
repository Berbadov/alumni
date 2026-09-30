use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

use actix_web::{delete, get, patch, post, put, web, HttpResponse};

use crate::error::ApiError;
use crate::models::{CreateUser, UpdateUser, User};

#[derive(Default)]
pub struct UserStore {
    next_id: AtomicU64,
    users: Mutex<BTreeMap<u64, User>>,
}

impl UserStore {
    fn lock(&self) -> Result<MutexGuard<'_, BTreeMap<u64, User>>, ApiError> {
        self.users
            .lock()
            .map_err(|_| ApiError::Internal("user store lock poisoned".to_owned()))
    }
}

fn validate_name(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        return Err(ApiError::BadRequest("name must not be empty".to_owned()));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), ApiError> {
    let Some((local, domain)) = email.trim().split_once('@') else {
        return Err(ApiError::BadRequest("email must contain '@'".to_owned()));
    };
    if local.is_empty() || domain.is_empty() {
        return Err(ApiError::BadRequest("invalid email format".to_owned()));
    }
    Ok(())
}

fn normalized(name: &str, email: &str) -> (String, String) {
    (name.trim().to_owned(), email.trim().to_lowercase())
}

/// Creates a user from the validated request body.
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 400, description = "Invalid name or email"),
    ),
)]
#[post("/api/users")]
pub async fn create_user(
    state: web::Data<UserStore>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();
    validate_name(&body.name)?;
    validate_email(&body.email)?;
    let id = state.next_id.fetch_add(1, Ordering::Relaxed) + 1;
    let (name, email) = normalized(&body.name, &body.email);
    let user = User { id, name, email };
    state.lock()?.insert(id, user.clone());
    Ok(HttpResponse::Created().json(user))
}

/// Lists all users, ordered by id.
#[utoipa::path(
    get,
    path = "/api/users",
    responses((status = 200, description = "All users", body = [User])),
)]
#[get("/api/users")]
pub async fn list_users(state: web::Data<UserStore>) -> Result<web::Json<Vec<User>>, ApiError> {
    Ok(web::Json(state.lock()?.values().cloned().collect()))
}

/// Returns one user by id.
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "The user", body = User),
        (status = 404, description = "No user with that id"),
    ),
)]
#[get("/api/users/{id}")]
pub async fn get_user(
    state: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<web::Json<User>, ApiError> {
    let id = path.into_inner();
    state
        .lock()?
        .get(&id)
        .cloned()
        .map(web::Json)
        .ok_or(ApiError::NotFound)
}

/// Replaces every field of a user (404 if the id does not exist).
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = CreateUser,
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "User replaced", body = User),
        (status = 400, description = "Invalid name or email"),
        (status = 404, description = "No user with that id"),
    ),
)]
#[put("/api/users/{id}")]
pub async fn replace_user(
    state: web::Data<UserStore>,
    path: web::Path<u64>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let body = body.into_inner();
    validate_name(&body.name)?;
    validate_email(&body.email)?;
    let mut users = state.lock()?;
    if !users.contains_key(&id) {
        return Err(ApiError::NotFound);
    }
    let (name, email) = normalized(&body.name, &body.email);
    let user = User { id, name, email };
    users.insert(id, user.clone());
    Ok(HttpResponse::Ok().json(user))
}

/// Applies a partial update to a user (only provided fields change).
#[utoipa::path(
    patch,
    path = "/api/users/{id}",
    request_body = UpdateUser,
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 400, description = "Invalid name or email"),
        (status = 404, description = "No user with that id"),
    ),
)]
#[patch("/api/users/{id}")]
pub async fn update_user(
    state: web::Data<UserStore>,
    path: web::Path<u64>,
    body: web::Json<UpdateUser>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let body = body.into_inner();
    if let Some(name) = &body.name {
        validate_name(name)?;
    }
    if let Some(email) = &body.email {
        validate_email(email)?;
    }
    let mut users = state.lock()?;
    let user = users.get_mut(&id).ok_or(ApiError::NotFound)?;
    if let Some(name) = body.name {
        name.trim().clone_into(&mut user.name);
    }
    if let Some(email) = body.email {
        email.trim().to_lowercase().clone_into(&mut user.email);
    }
    Ok(HttpResponse::Ok().json(user.clone()))
}

/// Deletes a user by id.
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "No user with that id"),
    ),
)]
#[delete("/api/users/{id}")]
pub async fn delete_user(
    state: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    if state.lock()?.remove(&id).is_some() {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(ApiError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use actix_web::App;
    use serde_json::json;

    #[actix_web::test]
    async fn create_get_patch_put_delete_lifecycle() {
        let app = test::init_service(
        App::new()
            .app_data(web::Data::new(UserStore::default()))
            .service(create_user)
            .service(list_users)
            .service(get_user)
            .service(replace_user)
            .service(update_user)
            .service(delete_user),
    ).await;

        let req = TestRequest::post()
            .uri("/api/users")
            .set_json(json!({ "name": "  Ada Lovelace  ", "email": "Ada@Example.com" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created: User = test::read_body_json(resp).await;
        assert_eq!(created.id, 1);
        assert_eq!(created.name, "Ada Lovelace");
        assert_eq!(created.email, "ada@example.com");

        let user: User =
            test::call_and_read_body_json(&app, TestRequest::get().uri("/api/users/1").to_request())
                .await;
        assert_eq!(user.name, "Ada Lovelace");

        let req = TestRequest::patch()
            .uri("/api/users/1")
            .set_json(json!({ "name": "Ada King" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let patched: User = test::read_body_json(resp).await;
        assert_eq!(patched.name, "Ada King");
        assert_eq!(patched.email, "ada@example.com");

        let req = TestRequest::put()
            .uri("/api/users/1")
            .set_json(json!({ "name": "Ada", "email": "ada@math.org" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: User = test::read_body_json(resp).await;
        assert_eq!(replaced.name, "Ada");
        assert_eq!(replaced.email, "ada@math.org");

        let req = TestRequest::delete().uri("/api/users/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::get().uri("/api/users/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn list_returns_all_in_id_order() {
        let app = test::init_service(
        App::new()
            .app_data(web::Data::new(UserStore::default()))
            .service(create_user)
            .service(list_users)
            .service(get_user)
            .service(replace_user)
            .service(update_user)
            .service(delete_user),
    ).await;

        for name in ["first", "second"] {
            let req = TestRequest::post()
                .uri("/api/users")
                .set_json(json!({ "name": name, "email": format!("{name}@example.com") }))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let users: Vec<User> =
            test::call_and_read_body_json(&app, TestRequest::get().uri("/api/users").to_request())
                .await;
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "first");
        assert_eq!(users[1].name, "second");
    }

    #[actix_web::test]
    async fn rejects_invalid_input() {
        let app = test::init_service(
        App::new()
            .app_data(web::Data::new(UserStore::default()))
            .service(create_user)
            .service(list_users)
            .service(get_user)
            .service(replace_user)
            .service(update_user)
            .service(delete_user),
    ).await;

        for body in [
            json!({ "name": "", "email": "a@example.com" }),
            json!({ "name": "Ada", "email": "not-an-email" }),
            json!({ "name": "Ada", "email": "@example.com" }),
        ] {
            let req = TestRequest::post().uri("/api/users").set_json(&body).to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        let req = TestRequest::patch()
            .uri("/api/users/999")
            .set_json(json!({ "name": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

