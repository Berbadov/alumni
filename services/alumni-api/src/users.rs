use actix_web::{HttpResponse, delete, get, patch, post, put, web};

use crate::error::ApiError;
use crate::models::{CreateUser, UpdateUser, User};
use crate::user_store::UserStore;

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
    store: web::Data<UserStore>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    let user = store.create(&body)?;
    Ok(HttpResponse::Created().json(user))
}

/// Lists all users, ordered by id.
#[utoipa::path(
    get,
    path = "/api/users",
    responses((status = 200, description = "All users", body = [User])),
)]
#[get("/api/users")]
pub async fn list_users(store: web::Data<UserStore>) -> Result<web::Json<Vec<User>>, ApiError> {
    Ok(web::Json(store.list()?))
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
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<web::Json<User>, ApiError> {
    Ok(web::Json(store.get(path.into_inner())?))
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
    store: web::Data<UserStore>,
    path: web::Path<u64>,
    body: web::Json<CreateUser>,
) -> Result<web::Json<User>, ApiError> {
    Ok(web::Json(store.replace(path.into_inner(), &body)?))
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
    path = "/api/users/{id}",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "No user with that id"),
    ),
)]
#[delete("/api/users/{id}")]
pub async fn delete_user(
    store: web::Data<UserStore>,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    store.delete(path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::App;
    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use serde_json::json;

    macro_rules! test_app {
        () => {
            test::init_service(
                App::new()
                    .app_data(web::Data::new(UserStore::default()))
                    .service(create_user)
                    .service(list_users)
                    .service(get_user)
                    .service(replace_user)
                    .service(update_user)
                    .service(delete_user),
            )
            .await
        };
    }

    #[actix_web::test]
    async fn create_get_patch_put_delete_lifecycle() {
        let app = test_app!();

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

        let user: User = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/users/1").to_request(),
        )
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

        let resp =
            test::call_service(&app, TestRequest::get().uri("/api/users/1").to_request()).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn list_returns_all_in_id_order() {
        let app = test_app!();

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
        let app = test_app!();

        for body in [
            json!({ "name": "", "email": "a@example.com" }),
            json!({ "name": "Ada", "email": "not-an-email" }),
            json!({ "name": "Ada", "email": "@example.com" }),
        ] {
            let req = TestRequest::post()
                .uri("/api/users")
                .set_json(&body)
                .to_request();
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
