pub mod announcement;
pub mod api_alumni;
pub mod api_announcement;
pub mod api_job_posting;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(api_alumni::configure)
        .configure(announcement::configure)
        .configure(api_announcement::configure)
        .configure(api_job_posting::configure);
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use actix_web::dev::ServiceResponse;
    use actix_web::http::StatusCode;
    use actix_web::http::header::{CONTENT_TYPE, ContentType, LOCATION};
    use actix_web::test::{self, TestRequest};
    use actix_web::{App, web};
    use mongodb::bson::{Document, doc};
    use mongodb::options::{ClientOptions, IndexOptions};
    use mongodb::{Client, IndexModel};
    use serde_json::json;

    use super::configure;
    use crate::alumni_store::AlumniStore;
    use crate::announcement_store::AnnouncementStore;
    use crate::job_posting_store::JobPostingStore;
    use crate::models::{Alumni, Announcement, JobPosting};
    use crate::AppState;

    static TEST_DB_COUNT: AtomicU64 = AtomicU64::new(0);

    fn test_uri() -> String {
        std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_owned())
    }

    async fn test_client() -> Client {
        let options = ClientOptions::parse(test_uri()).await.unwrap();
        Client::with_options(options).unwrap()
    }

    async fn test_state() -> (AppState, String) {
        let n = TEST_DB_COUNT.fetch_add(1, Ordering::Relaxed);
        let name = format!("alumni_test_{}_{}", std::process::id(), n);
        let db = test_client().await.database(&name);
        db.collection::<Document>("alumni")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "email": 1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .unwrap();
        db.collection::<Document>("announcements")
            .create_index(IndexModel::builder().keys(doc! { "author_id": 1 }).build())
            .await
            .unwrap();
        db.collection::<Document>("job_postings")
            .create_index(IndexModel::builder().keys(doc! { "author_id": 1 }).build())
            .await
            .unwrap();
        db.collection::<Document>("job_postings")
            .create_index(IndexModel::builder().keys(doc! { "is_active": 1 }).build())
            .await
            .unwrap();
        (
            AppState {
                alumni: AlumniStore::new(&db),
                announcements: AnnouncementStore::new(&db),
                job_postings: JobPostingStore::new(&db),
            },
            name,
        )
    }

    async fn cleanup(name: &str) {
        test_client().await.database(name).drop().await.unwrap();
    }

    macro_rules! test_app {
        () => {{
            let (state, db_name) = test_state().await;
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(state))
                    .configure(configure),
            )
            .await;
            (app, db_name)
        }};
    }

    fn alumni_json(name: &str, email: &str) -> serde_json::Value {
        json!({
            "full_name": name,
            "email": email,
            "graduation_year": 2027,
            "degree": "Web Development",
        })
    }

    macro_rules! seed_alumni {
        ($app:expr) => {
            let req = TestRequest::post()
                .uri("/api/v1/alumni")
                .set_json(alumni_json("Ada Lovelace", "ada@example.com"))
                .to_request();
            assert_eq!(
                test::call_service(&$app, req).await.status(),
                StatusCode::CREATED
            );
        };
    }

    macro_rules! seed_announcement {
        ($app:expr) => {
            seed_alumni!($app);
            let req = TestRequest::post()
                .uri("/api/v1/announcements")
                .set_json(json!({ "title": "Reunion", "body": "Save the date", "author_id": 1 }))
                .to_request();
            assert_eq!(
                test::call_service(&$app, req).await.status(),
                StatusCode::CREATED
            );
        };
    }

    fn job_posting_json(title: &str) -> serde_json::Value {
        json!({
            "title": title,
            "company": "Acme",
            "description": "Build things",
            "location": "Remote",
            "employment_type": "Full-time",
            "apply_url": "https://example.com/jobs/1",
            "author_id": 1,
        })
    }

    macro_rules! seed_job_posting {
        ($app:expr) => {
            seed_alumni!($app);
            let req = TestRequest::post()
                .uri("/api/v1/job-postings")
                .set_json(job_posting_json("Rust Engineer"))
                .to_request();
            assert_eq!(
                test::call_service(&$app, req).await.status(),
                StatusCode::CREATED
            );
        };
    }

    #[actix_web::test]
    async fn alumni_create_get_patch_put_delete_lifecycle() {
        let (app, db) = test_app!();

        let req = TestRequest::post()
            .uri("/api/v1/alumni")
            .set_json(alumni_json("  Ada Lovelace  ", "Ada@Example.com"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created: Alumni = test::read_body_json(resp).await;
        assert_eq!(created.id, 1);
        assert_eq!(created.full_name, "Ada Lovelace");
        assert_eq!(created.email, "ada@example.com");
        assert_eq!(created.graduation_year, 2027);

        let uri = "/api/v1/alumni/1";
        let alumni: Alumni =
            test::call_and_read_body_json(&app, TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(alumni.full_name, "Ada Lovelace");

        let req = TestRequest::patch()
            .uri(uri)
            .set_json(json!({ "full_name": "Ada King" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let patched: Alumni = test::read_body_json(resp).await;
        assert_eq!(patched.full_name, "Ada King");
        assert_eq!(patched.email, "ada@example.com");

        let req = TestRequest::put()
            .uri(uri)
            .set_json(alumni_json("Ada", "ada@math.org"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: Alumni = test::read_body_json(resp).await;
        assert_eq!(replaced.full_name, "Ada");
        assert_eq!(replaced.email, "ada@math.org");

        let resp =
            test::call_service(&app, TestRequest::delete().uri(uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(&app, TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_list_returns_all_in_id_order() {
        let (app, db) = test_app!();

        for (name, email) in [("first", "first@example.com"), ("second", "second@example.com")] {
            let req = TestRequest::post()
                .uri("/api/v1/alumni")
                .set_json(alumni_json(name, email))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let alumni: Vec<Alumni> = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/alumni").to_request(),
        )
        .await;
        assert_eq!(alumni.len(), 2);
        assert_eq!(alumni[0].full_name, "first");
        assert_eq!(alumni[1].full_name, "second");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_rejects_invalid_input() {
        let (app, db) = test_app!();

        for body in [
            alumni_json("", "a@example.com"),
            alumni_json("Ada", "not-an-email"),
            alumni_json("Ada", "@example.com"),
            json!({
                "full_name": "Ada",
                "email": "ada@example.com",
                "graduation_year": 1800,
                "degree": "Web Development",
            }),
            json!({
                "full_name": "Ada",
                "email": "ada@example.com",
                "graduation_year": 2027,
                "degree": "",
            }),
        ] {
            let req = TestRequest::post()
                .uri("/api/v1/alumni")
                .set_json(&body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        let req = TestRequest::patch()
            .uri("/api/v1/alumni/999")
            .set_json(json!({ "full_name": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_rejects_duplicate_email() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        let req = TestRequest::post()
            .uri("/api/v1/alumni")
            .set_json(alumni_json("Ada Clone", "ADA@example.com"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_delete_with_announcements_fails_until_they_go() {
        let (app, db) = test_app!();
        seed_announcement!(app);

        let resp = test::call_service(
            &app,
            TestRequest::delete().uri("/api/v1/alumni/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/announcements/1")
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::delete().uri("/api/v1/alumni/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_create_get_patch_put_delete_lifecycle() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        let req = TestRequest::post()
            .uri("/api/v1/announcements")
            .set_json(json!({ "title": "  Reunion  ", "body": "  Save the date  ", "author_id": 1 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created: Announcement = test::read_body_json(resp).await;
        assert_eq!(created.id, 1);
        assert_eq!(created.title, "Reunion");
        assert_eq!(created.body, "Save the date");
        assert_eq!(created.author_id, 1);

        let uri = "/api/v1/announcements/1";
        let announcement: Announcement = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri(uri).to_request(),
        )
        .await;
        assert_eq!(announcement.title, "Reunion");

        let req = TestRequest::patch()
            .uri(uri)
            .set_json(json!({ "title": "Picnic" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let patched: Announcement = test::read_body_json(resp).await;
        assert_eq!(patched.title, "Picnic");
        assert_eq!(patched.body, "Save the date");

        let req = TestRequest::put()
            .uri(uri)
            .set_json(json!({ "title": "Fair", "body": "New body", "author_id": 1 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: Announcement = test::read_body_json(resp).await;
        assert_eq!(replaced.title, "Fair");
        assert_eq!(replaced.author_id, 1);

        let resp = test::call_service(&app, TestRequest::delete().uri(uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(&app, TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_list_returns_all_in_id_order() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        for title in ["first", "second"] {
            let req = TestRequest::post()
                .uri("/api/v1/announcements")
                .set_json(json!({ "title": title, "body": "body", "author_id": 1 }))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let announcements: Vec<Announcement> = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/announcements").to_request(),
        )
        .await;
        assert_eq!(announcements.len(), 2);
        assert_eq!(announcements[0].title, "first");
        assert_eq!(announcements[1].title, "second");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_rejects_invalid_input() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        for body in [
            json!({ "title": "", "body": "body", "author_id": 1 }),
            json!({ "title": "Title", "body": "", "author_id": 1 }),
            json!({ "title": "Title", "body": "body", "author_id": 999 }),
        ] {
            let req = TestRequest::post()
                .uri("/api/v1/announcements")
                .set_json(&body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        let req = TestRequest::patch()
            .uri("/api/v1/announcements/999")
            .set_json(json!({ "title": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_alias_serves_the_same_store() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        let req = TestRequest::post()
            .uri("/api/announcements")
            .set_json(json!({ "title": "Alias", "body": "body", "author_id": 1 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let announcements: Vec<Announcement> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/announcements")
                .to_request(),
        )
        .await;
        assert_eq!(announcements.len(), 1);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_create_get_patch_put_delete_lifecycle() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        let req = TestRequest::post()
            .uri("/api/v1/job-postings")
            .set_json(json!({
                "title": "  Rust Engineer  ",
                "company": "  Acme  ",
                "description": "  Build APIs  ",
                "location": "  Remote  ",
                "employment_type": "  Full-time  ",
                "apply_url": "  https://example.com/jobs/1  ",
                "author_id": 1,
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created: JobPosting = test::read_body_json(resp).await;
        assert_eq!(created.id, 1);
        assert_eq!(created.title, "Rust Engineer");
        assert_eq!(created.company, "Acme");
        assert_eq!(created.description, "Build APIs");
        assert_eq!(created.location, "Remote");
        assert_eq!(created.employment_type, "Full-time");
        assert_eq!(created.apply_url, "https://example.com/jobs/1");
        assert_eq!(created.author_id, 1);
        assert!(created.is_active);

        let uri = "/api/v1/job-postings/1";
        let posting: JobPosting = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri(uri).to_request(),
        )
        .await;
        assert_eq!(posting.title, "Rust Engineer");

        let req = TestRequest::patch()
            .uri(uri)
            .set_json(json!({ "title": "Backend Engineer", "is_active": false }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let patched: JobPosting = test::read_body_json(resp).await;
        assert_eq!(patched.title, "Backend Engineer");
        assert!(!patched.is_active);
        assert_eq!(patched.company, "Acme");

        let req = TestRequest::put()
            .uri(uri)
            .set_json(job_posting_json("Platform Engineer"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: JobPosting = test::read_body_json(resp).await;
        assert_eq!(replaced.title, "Platform Engineer");
        assert!(replaced.is_active);

        let resp = test::call_service(&app, TestRequest::delete().uri(uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(&app, TestRequest::get().uri(uri).to_request()).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_list_returns_id_order_and_filters_active() {
        let (app, db) = test_app!();
        seed_alumni!(app);

        for (title, active) in [("Active", true), ("Inactive", false)] {
            let mut body = job_posting_json(title);
            body["is_active"] = json!(active);
            let req = TestRequest::post()
                .uri("/api/v1/job-postings")
                .set_json(body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let all: Vec<JobPosting> = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/job-postings").to_request(),
        )
        .await;
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].title, "Active");
        assert_eq!(all[1].title, "Inactive");

        let active: Vec<JobPosting> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/job-postings?active=true")
                .to_request(),
        )
        .await;
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].title, "Active");

        let inactive: Vec<JobPosting> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/job-postings?active=false")
                .to_request(),
        )
        .await;
        assert_eq!(inactive.len(), 1);
        assert_eq!(inactive[0].title, "Inactive");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_rejects_invalid_input() {
        let (app, db) = test_app!();
        seed_job_posting!(app);

        for (field, value) in [
            ("title", json!("")),
            ("company", json!("  ")),
            ("description", json!("")),
            ("location", json!("")),
            ("employment_type", json!(" ")),
            ("apply_url", json!("")),
            ("apply_url", json!("ftp://example.com")),
            ("author_id", json!(999)),
        ] {
            let mut body = job_posting_json("Engineer");
            body[field] = value;
            let req = TestRequest::post()
                .uri("/api/v1/job-postings")
                .set_json(&body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "{field}");
        }

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/1")
            .set_json(json!({ "apply_url": "example.com" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/1")
            .set_json(json!({ "author_id": 999 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/999")
            .set_json(json!({ "title": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_delete_with_job_postings_fails_until_they_go() {
        let (app, db) = test_app!();
        seed_job_posting!(app);

        let resp = test::call_service(
            &app,
            TestRequest::delete().uri("/api/v1/alumni/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/job-postings/1")
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::delete().uri("/api/v1/alumni/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        cleanup(&db).await;
    }

    async fn body_text(resp: ServiceResponse) -> String {
        String::from_utf8(test::read_body(resp).await.to_vec()).unwrap()
    }

    #[actix_web::test]
    async fn announcements_page_lists_and_escapes_html() {
        let (app, db) = test_app!();
        seed_alumni!(app);
        let req = TestRequest::post()
            .uri("/api/v1/announcements")
            .set_json(json!({ "title": "<b>Reunion</b>", "body": "Save the date", "author_id": 1 }))
            .to_request();
        assert_eq!(
            test::call_service(&app, req).await.status(),
            StatusCode::CREATED
        );

        let resp =
            test::call_service(&app, TestRequest::get().uri("/announcements").to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let content_type = resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
        assert!(content_type.starts_with("text/html"));
        let body = body_text(resp).await;
        assert!(body.contains("&lt;b&gt;Reunion&lt;/b&gt;"));
        assert!(!body.contains("<b>Reunion</b>"));

        cleanup(&db).await;
    }

    fn announcement_form_post(uri: &str, payload: &str) -> TestRequest {
        TestRequest::post()
            .uri(uri)
            .insert_header(ContentType::form_url_encoded())
            .set_payload(payload.to_owned())
    }

    #[actix_web::test]
    async fn announcement_form_post_creates_and_redirects() {
        let (app, db) = test_app!();
        seed_alumni!(app);
        let req =
            announcement_form_post("/announcements", "title=Reunion&body=Save+the+date&author_id=1")
                .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/announcements");

        let announcement: Announcement = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/announcements/1").to_request(),
        )
        .await;
        assert_eq!(announcement.title, "Reunion");
        assert_eq!(announcement.body, "Save the date");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_with_invalid_input_shows_the_error() {
        let (app, db) = test_app!();
        seed_alumni!(app);
        let req = announcement_form_post(
            "/announcements",
            "title=&body=Save+the+date&author_id=1",
        )
        .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_text(resp).await;
        assert!(body.contains("title must not be empty"));
        assert!(body.contains("Save the date"));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_with_unknown_author_shows_the_error() {
        let (app, db) = test_app!();
        let req = announcement_form_post(
            "/announcements",
            "title=Reunion&body=Save+the+date&author_id=999",
        )
        .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_text(resp).await;
        assert!(body.contains("unknown author_id"));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcements_index_page_links_each_row_to_show_edit_and_delete() {
        let (app, db) = test_app!();
        seed_announcement!(app);

        let resp =
            test::call_service(&app, TestRequest::get().uri("/announcements").to_request()).await;
        let body = body_text(resp).await;
        assert!(body.contains(r#"href="/announcements/1""#));
        assert!(body.contains(r#"href="/announcements/1/edit""#));
        assert!(body.contains(r#"action="/announcements/1/delete""#));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_show_and_edit_pages_render_the_announcement() {
        let (app, db) = test_app!();
        seed_announcement!(app);

        let resp = test::call_service(
            &app,
            TestRequest::get().uri("/announcements/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_text(resp).await;
        assert!(body.contains("Save the date"));
        assert!(body.contains(r#"href="/announcements/1/edit""#));

        let resp = test::call_service(
            &app,
            TestRequest::get().uri("/announcements/1/edit").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_text(resp).await;
        assert!(body.contains(r#"value="Reunion""#));
        assert!(body.contains(r#"action="/announcements/1""#));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_updates_and_redirects() {
        let (app, db) = test_app!();
        seed_announcement!(app);

        let resp = test::call_service(
            &app,
            announcement_form_post(
                "/announcements/1",
                "title=Picnic&body=Bring+food&author_id=1",
            )
            .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/announcements/1");

        let announcement: Announcement = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/v1/announcements/1").to_request(),
        )
        .await;
        assert_eq!(announcement.title, "Picnic");
        assert_eq!(announcement.author_id, 1);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_update_with_invalid_input_shows_the_edit_page_with_error() {
        let (app, db) = test_app!();
        seed_announcement!(app);

        let resp = test::call_service(
            &app,
            announcement_form_post("/announcements/1", "title=&body=Bring+food&author_id=1")
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_text(resp).await;
        assert!(body.contains("title must not be empty"));
        assert!(body.contains("Bring food"));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_deletes_and_redirects() {
        let (app, db) = test_app!();
        seed_announcement!(app);

        let resp = test::call_service(
            &app,
            announcement_form_post("/announcements/1/delete", "").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/announcements");

        let resp = test::call_service(
            &app,
            TestRequest::get().uri("/api/v1/announcements/1").to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn missing_announcement_renders_an_html_404_page() {
        let (app, db) = test_app!();

        let requests = [
            TestRequest::get().uri("/announcements/99").to_request(),
            TestRequest::get()
                .uri("/announcements/99/edit")
                .to_request(),
            announcement_form_post("/announcements/99", "title=T&body=B&author_id=1").to_request(),
            announcement_form_post("/announcements/99/delete", "").to_request(),
        ];
        for req in requests {
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let content_type = resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
            assert!(content_type.starts_with("text/html"));
        }

        cleanup(&db).await;
    }
}
