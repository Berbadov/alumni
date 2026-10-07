pub mod api_user_controller;
pub mod user_controller;

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use actix_web::{App, web};
    use serde_json::json;

    use super::{api_user_controller, user_controller};
    use crate::models::User;
    use crate::user_store::UserStore;

    macro_rules! test_app {
        () => {
            test::init_service(
                App::new()
                    .app_data(web::Data::new(UserStore::default()))
                    .configure(user_controller::configure)
                    .service(web::scope("/api/v1").configure(api_user_controller::configure)),
            )
            .await
        };
    }

    macro_rules! controller_tests {
        ($module:ident, $base:literal) => {
            mod $module {
                use super::*;

                #[actix_web::test]
                async fn create_get_patch_put_delete_lifecycle() {
                    let app = test_app!();
                    let base = $base;

                    let req = TestRequest::post()
                        .uri(base)
                        .set_json(json!({ "name": "  Ada Lovelace  ", "email": "Ada@Example.com" }))
                        .to_request();
                    let resp = test::call_service(&app, req).await;
                    assert_eq!(resp.status(), StatusCode::CREATED);
                    let created: User = test::read_body_json(resp).await;
                    assert_eq!(created.id, 1);
                    assert_eq!(created.name, "Ada Lovelace");
                    assert_eq!(created.email, "ada@example.com");

                    let uri = format!("{base}/1");
                    let user: User = test::call_and_read_body_json(
                        &app,
                        TestRequest::get().uri(&uri).to_request(),
                    )
                    .await;
                    assert_eq!(user.name, "Ada Lovelace");

                    let req = TestRequest::patch()
                        .uri(&uri)
                        .set_json(json!({ "name": "Ada King" }))
                        .to_request();
                    let resp = test::call_service(&app, req).await;
                    assert_eq!(resp.status(), StatusCode::OK);
                    let patched: User = test::read_body_json(resp).await;
                    assert_eq!(patched.name, "Ada King");
                    assert_eq!(patched.email, "ada@example.com");

                    let req = TestRequest::put()
                        .uri(&uri)
                        .set_json(json!({ "name": "Ada", "email": "ada@math.org" }))
                        .to_request();
                    let resp = test::call_service(&app, req).await;
                    assert_eq!(resp.status(), StatusCode::OK);
                    let replaced: User = test::read_body_json(resp).await;
                    assert_eq!(replaced.name, "Ada");
                    assert_eq!(replaced.email, "ada@math.org");

                    let resp = test::call_service(
                        &app,
                        TestRequest::delete().uri(&uri).to_request(),
                    )
                    .await;
                    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

                    let resp =
                        test::call_service(&app, TestRequest::get().uri(&uri).to_request()).await;
                    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
                }

                #[actix_web::test]
                async fn list_returns_all_in_id_order() {
                    let app = test_app!();
                    let base = $base;

                    for name in ["first", "second"] {
                        let req = TestRequest::post()
                            .uri(base)
                            .set_json(json!({ "name": name, "email": format!("{name}@example.com") }))
                            .to_request();
                        let resp = test::call_service(&app, req).await;
                        assert_eq!(resp.status(), StatusCode::CREATED);
                    }

                    let users: Vec<User> = test::call_and_read_body_json(
                        &app,
                        TestRequest::get().uri(base).to_request(),
                    )
                    .await;
                    assert_eq!(users.len(), 2);
                    assert_eq!(users[0].name, "first");
                    assert_eq!(users[1].name, "second");
                }

                #[actix_web::test]
                async fn rejects_invalid_input() {
                    let app = test_app!();
                    let base = $base;

                    for body in [
                        json!({ "name": "", "email": "a@example.com" }),
                        json!({ "name": "Ada", "email": "not-an-email" }),
                        json!({ "name": "Ada", "email": "@example.com" }),
                    ] {
                        let req = TestRequest::post().uri(base).set_json(&body).to_request();
                        let resp = test::call_service(&app, req).await;
                        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
                    }

                    let req = TestRequest::patch()
                        .uri(&format!("{base}/999"))
                        .set_json(json!({ "name": "Ghost" }))
                        .to_request();
                    let resp = test::call_service(&app, req).await;
                    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
                }
            }
        };
    }

    controller_tests!(unversioned, "/api/users");
    controller_tests!(versioned, "/api/v1/users");

    #[actix_web::test]
    async fn both_controllers_share_one_store() {
        let app = test_app!();

        let req = TestRequest::post()
            .uri("/api/users")
            .set_json(json!({ "name": "Ada", "email": "ada@example.com" }))
            .to_request();
        assert_eq!(
            test::call_service(&app, req).await.status(),
            StatusCode::CREATED
        );

        let user: User = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/users/1").to_request(),
        )
        .await;
        assert_eq!(user.name, "Ada");
    }
}
