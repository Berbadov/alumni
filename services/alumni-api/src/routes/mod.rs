pub mod api_user;
pub mod user;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(user::configure)
        .configure(api_user::configure);
}

#[cfg(test)]
mod tests {
    use actix_web::dev::ServiceResponse;
    use actix_web::http::StatusCode;
    use actix_web::http::header::{CONTENT_TYPE, ContentType, LOCATION};
    use actix_web::test::{self, TestRequest};
    use actix_web::{App, web};
    use serde_json::json;

    use super::configure;
    use crate::models::User;
    use crate::user_store::UserStore;

    macro_rules! test_app {
        () => {
            test::init_service(
                App::new()
                    .app_data(web::Data::new(UserStore::default()))
                    .configure(configure),
            )
            .await
        };
    }

    macro_rules! api_tests {
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

                    let resp =
                        test::call_service(&app, TestRequest::delete().uri(&uri).to_request()).await;
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

    api_tests!(versioned, "/api/v1/users");
    api_tests!(unversioned, "/api/users");

    async fn body_text(resp: ServiceResponse) -> String {
        String::from_utf8(test::read_body(resp).await.to_vec()).unwrap()
    }

    #[actix_web::test]
    async fn users_page_lists_users_and_escapes_html() {
        let app = test_app!();
        let req = TestRequest::post()
            .uri("/api/v1/users")
            .set_json(json!({ "name": "<b>Ada</b>", "email": "ada@example.com" }))
            .to_request();
        assert_eq!(
            test::call_service(&app, req).await.status(),
            StatusCode::CREATED
        );

        let resp = test::call_service(&app, TestRequest::get().uri("/users").to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let content_type = resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
        assert!(content_type.starts_with("text/html"));
        let body = body_text(resp).await;
        assert!(body.contains("&lt;b&gt;Ada&lt;/b&gt;"));
        assert!(!body.contains("<b>Ada</b>"));
    }

    #[actix_web::test]
    async fn form_post_creates_user_and_redirects() {
        let app = test_app!();
        let req = TestRequest::post()
            .uri("/users")
            .insert_header(ContentType::form_url_encoded())
            .set_payload("name=Ada+Lovelace&email=ada%40example.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/users");

        let user: User = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/users/1").to_request(),
        )
        .await;
        assert_eq!(user.name, "Ada Lovelace");
    }

    #[actix_web::test]
    async fn form_post_with_invalid_input_shows_the_error() {
        let app = test_app!();
        let req = TestRequest::post()
            .uri("/users")
            .insert_header(ContentType::form_url_encoded())
            .set_payload("name=&email=ada%40example.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_text(resp).await;
        assert!(body.contains("name must not be empty"));
        assert!(body.contains(r#"value="ada@example.com""#));
    }

    fn form_post(uri: &str, payload: &str) -> TestRequest {
        TestRequest::post()
            .uri(uri)
            .insert_header(ContentType::form_url_encoded())
            .set_payload(payload.to_owned())
    }

    macro_rules! seed_ada {
        ($app:expr) => {
            let req = TestRequest::post()
                .uri("/api/v1/users")
                .set_json(json!({ "name": "Ada", "email": "ada@example.com" }))
                .to_request();
            assert_eq!(test::call_service(&$app, req).await.status(), StatusCode::CREATED);
        };
    }

    #[actix_web::test]
    async fn index_page_links_each_user_to_show_edit_and_delete() {
        let app = test_app!();
        seed_ada!(app);

        let resp = test::call_service(&app, TestRequest::get().uri("/users").to_request()).await;
        let body = body_text(resp).await;
        assert!(body.contains(r#"href="/users/1""#));
        assert!(body.contains(r#"href="/users/1/edit""#));
        assert!(body.contains(r#"action="/users/1/delete""#));
    }

    #[actix_web::test]
    async fn show_and_edit_pages_render_the_user() {
        let app = test_app!();
        seed_ada!(app);

        let resp = test::call_service(&app, TestRequest::get().uri("/users/1").to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_text(resp).await;
        assert!(body.contains("ada@example.com"));
        assert!(body.contains(r#"href="/users/1/edit""#));

        let resp =
            test::call_service(&app, TestRequest::get().uri("/users/1/edit").to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_text(resp).await;
        assert!(body.contains(r#"value="Ada""#));
        assert!(body.contains(r#"action="/users/1""#));
    }

    #[actix_web::test]
    async fn form_post_updates_user_and_redirects() {
        let app = test_app!();
        seed_ada!(app);

        let resp = test::call_service(
            &app,
            form_post("/users/1", "name=Ada+King&email=king%40example.com").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/users/1");

        let user: User = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/users/1").to_request(),
        )
        .await;
        assert_eq!(
            (user.name.as_str(), user.email.as_str()),
            ("Ada King", "king@example.com")
        );
    }

    #[actix_web::test]
    async fn form_post_update_with_invalid_input_shows_the_edit_page_with_error() {
        let app = test_app!();
        seed_ada!(app);

        let resp = test::call_service(
            &app,
            form_post("/users/1", "name=&email=king%40example.com").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_text(resp).await;
        assert!(body.contains("name must not be empty"));
        assert!(body.contains(r#"value="king@example.com""#));
    }

    #[actix_web::test]
    async fn form_post_deletes_user_and_redirects() {
        let app = test_app!();
        seed_ada!(app);

        let resp = test::call_service(&app, form_post("/users/1/delete", "").to_request()).await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/users");

        let resp =
            test::call_service(&app, TestRequest::get().uri("/api/v1/users/1").to_request()).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn missing_user_renders_an_html_404_page() {
        let app = test_app!();

        let requests = [
            TestRequest::get().uri("/users/99").to_request(),
            TestRequest::get().uri("/users/99/edit").to_request(),
            form_post("/users/99", "name=Ada&email=ada%40example.com").to_request(),
            form_post("/users/99/delete", "").to_request(),
        ];
        for req in requests {
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let content_type = resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
            assert!(content_type.starts_with("text/html"));
        }
    }
}
