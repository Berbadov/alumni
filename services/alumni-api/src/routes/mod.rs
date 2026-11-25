pub mod announcement;
pub mod api_admin;
pub mod api_alumni;
pub mod api_announcement;
pub mod api_auth;
pub mod api_job_posting;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(api_admin::configure)
        .configure(api_alumni::configure)
        .configure(api_auth::configure)
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
    use crate::auth_middleware::AuthMiddleware;
    use crate::job_posting_store::JobPostingStore;
    use crate::models::{
        Alumni, Announcement, AuthSession, JobPosting, Role, User, VerificationStatus,
    };
    use crate::session_store::SessionStore;
    use crate::user_store::UserStore;
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
        db.collection::<Document>("users")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "email": 1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .unwrap();
        db.collection::<Document>("users")
            .create_index(IndexModel::builder().keys(doc! { "roles": 1 }).build())
            .await
            .unwrap();
        db.collection::<Document>("sessions")
            .create_index(IndexModel::builder().keys(doc! { "user_id": 1 }).build())
            .await
            .unwrap();
        db.collection::<Document>("sessions")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "expires_at": 1 })
                    .options(
                        IndexOptions::builder()
                            .expire_after(std::time::Duration::from_secs(0))
                            .build(),
                    )
                    .build(),
            )
            .await
            .unwrap();
        (
            AppState {
                alumni: AlumniStore::new(&db),
                announcements: AnnouncementStore::new(&db),
                job_postings: JobPostingStore::new(&db),
                users: UserStore::new(&db),
                sessions: SessionStore::new(&db),
            },
            name,
        )
    }

    async fn cleanup(name: &str) {
        test_client().await.database(name).drop().await.unwrap();
    }

    async fn session_for(
        db_name: &str,
        email: &str,
        role: Role,
    ) -> (u64, actix_web::cookie::Cookie<'static>) {
        use mongodb::bson::DateTime;
        use mongodb::options::ReturnDocument;

        let db = test_client().await.database(db_name);
        let updated = db
            .collection::<Document>("counters")
            .find_one_and_update(
                doc! { "_id": "users" },
                doc! { "$inc": { "seq": 1_i64 } },
            )
            .upsert(true)
            .return_document(ReturnDocument::After)
            .await
            .unwrap()
            .unwrap();
        let raw_id = updated.get_i64("seq").unwrap();
        let user_id = u64::try_from(raw_id).unwrap();
        let now = DateTime::now();
        let status = if role == Role::Unverified {
            "pending"
        } else {
            "verified"
        };
        db.collection::<Document>("users")
            .insert_one(doc! {
                "_id": raw_id,
                "first_name": "Test",
                "last_name": "User",
                "full_name": "Test User",
                "age": 30_i32,
                "email": email,
                "roles": [role.as_str()],
                "verification_status": status,
                "email_verified": false,
                "phone_verified": false,
                "is_active": true,
                "created_at": now,
                "updated_at": now,
            })
            .await
            .unwrap();
        let (token, _) = SessionStore::new(&db).create(user_id).await.unwrap();
        (user_id, actix_web::cookie::Cookie::new("alumni_session", token))
    }

    macro_rules! test_app {
        () => {{
            let (state, db_name) = test_state().await;
            let app = test::init_service(
                App::new()
                    .app_data(web::Data::new(state))
                    .wrap(AuthMiddleware)
                    .configure(configure),
            )
            .await;
            (app, db_name)
        }};
    }

    macro_rules! admin_app {
        () => {{
            let (app, db_name) = test_app!();
            let (_, admin) = session_for(&db_name, "admin@test.edu", Role::Admin).await;
            (app, db_name, admin)
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
        ($app:expr, $cookie:expr) => {
            let req = TestRequest::post()
                .uri("/api/v1/alumni")
                .cookie($cookie.clone())
                .set_json(alumni_json("Ada Lovelace", "ada@example.com"))
                .to_request();
            assert_eq!(
                test::call_service(&$app, req).await.status(),
                StatusCode::CREATED
            );
        };
    }

    macro_rules! seed_announcement {
        ($app:expr, $cookie:expr) => {
            seed_alumni!($app, $cookie);
            let req = TestRequest::post()
                .uri("/api/v1/announcements")
                .cookie($cookie.clone())
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
        ($app:expr, $cookie:expr) => {
            seed_alumni!($app, $cookie);
            let req = TestRequest::post()
                .uri("/api/v1/job-postings")
                .cookie($cookie.clone())
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
        let (app, db, admin) = admin_app!();

        let req = TestRequest::post()
            .uri("/api/v1/alumni")
            .cookie(admin.clone())
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
        let alumni: Alumni = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri(uri).cookie(admin.clone()).to_request(),
        )
        .await;
        assert_eq!(alumni.full_name, "Ada Lovelace");

        let req = TestRequest::patch()
            .uri(uri)
            .cookie(admin.clone())
            .set_json(json!({ "full_name": "Ada King" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let patched: Alumni = test::read_body_json(resp).await;
        assert_eq!(patched.full_name, "Ada King");
        assert_eq!(patched.email, "ada@example.com");

        let req = TestRequest::put()
            .uri(uri)
            .cookie(admin.clone())
            .set_json(alumni_json("Ada", "ada@math.org"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: Alumni = test::read_body_json(resp).await;
        assert_eq!(replaced.full_name, "Ada");
        assert_eq!(replaced.email, "ada@math.org");

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri(uri)
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::get().uri(uri).cookie(admin).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_list_returns_all_in_id_order() {
        let (app, db, admin) = admin_app!();

        for (name, email) in [("first", "first@example.com"), ("second", "second@example.com")] {
            let req = TestRequest::post()
                .uri("/api/v1/alumni")
                .cookie(admin.clone())
                .set_json(alumni_json(name, email))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let alumni: Vec<Alumni> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/alumni")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(alumni.len(), 2);
        assert_eq!(alumni[0].full_name, "first");
        assert_eq!(alumni[1].full_name, "second");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_rejects_invalid_input() {
        let (app, db, admin) = admin_app!();

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
                .cookie(admin.clone())
                .set_json(&body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        let req = TestRequest::patch()
            .uri("/api/v1/alumni/999")
            .cookie(admin)
            .set_json(json!({ "full_name": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_rejects_duplicate_email() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        let req = TestRequest::post()
            .uri("/api/v1/alumni")
            .cookie(admin)
            .set_json(alumni_json("Ada Clone", "ADA@example.com"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_delete_with_announcements_fails_until_they_go() {
        let (app, db, admin) = admin_app!();
        seed_announcement!(app, admin);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/alumni/1")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/announcements/1")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/alumni/1")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_create_get_patch_put_delete_lifecycle() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        let req = TestRequest::post()
            .uri("/api/v1/announcements")
            .cookie(admin.clone())
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
            TestRequest::get().uri(uri).cookie(admin.clone()).to_request(),
        )
        .await;
        assert_eq!(announcement.title, "Reunion");

        let req = TestRequest::patch()
            .uri(uri)
            .cookie(admin.clone())
            .set_json(json!({ "title": "Picnic" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let patched: Announcement = test::read_body_json(resp).await;
        assert_eq!(patched.title, "Picnic");
        assert_eq!(patched.body, "Save the date");

        let req = TestRequest::put()
            .uri(uri)
            .cookie(admin.clone())
            .set_json(json!({ "title": "Fair", "body": "New body", "author_id": 1 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: Announcement = test::read_body_json(resp).await;
        assert_eq!(replaced.title, "Fair");
        assert_eq!(replaced.author_id, 1);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri(uri)
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::get().uri(uri).cookie(admin).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_list_returns_all_in_id_order() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        for title in ["first", "second"] {
            let req = TestRequest::post()
                .uri("/api/v1/announcements")
                .cookie(admin.clone())
                .set_json(json!({ "title": title, "body": "body", "author_id": 1 }))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let announcements: Vec<Announcement> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/announcements")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(announcements.len(), 2);
        assert_eq!(announcements[0].title, "first");
        assert_eq!(announcements[1].title, "second");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_rejects_invalid_input() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        for body in [
            json!({ "title": "", "body": "body", "author_id": 1 }),
            json!({ "title": "Title", "body": "", "author_id": 1 }),
            json!({ "title": "Title", "body": "body", "author_id": 999 }),
        ] {
            let req = TestRequest::post()
                .uri("/api/v1/announcements")
                .cookie(admin.clone())
                .set_json(&body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        let req = TestRequest::patch()
            .uri("/api/v1/announcements/999")
            .cookie(admin)
            .set_json(json!({ "title": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_alias_serves_the_same_store() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        let req = TestRequest::post()
            .uri("/api/announcements")
            .cookie(admin.clone())
            .set_json(json!({ "title": "Alias", "body": "body", "author_id": 1 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let announcements: Vec<Announcement> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/announcements")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(announcements.len(), 1);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_create_get_patch_put_delete_lifecycle() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        let req = TestRequest::post()
            .uri("/api/v1/job-postings")
            .cookie(admin.clone())
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
            TestRequest::get().uri(uri).cookie(admin.clone()).to_request(),
        )
        .await;
        assert_eq!(posting.title, "Rust Engineer");

        let req = TestRequest::patch()
            .uri(uri)
            .cookie(admin.clone())
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
            .cookie(admin.clone())
            .set_json(job_posting_json("Platform Engineer"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let replaced: JobPosting = test::read_body_json(resp).await;
        assert_eq!(replaced.title, "Platform Engineer");
        assert!(replaced.is_active);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri(uri)
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::get().uri(uri).cookie(admin).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_list_returns_id_order_and_filters_active() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        for (title, active) in [("Active", true), ("Inactive", false)] {
            let mut body = job_posting_json(title);
            body["is_active"] = json!(active);
            let req = TestRequest::post()
                .uri("/api/v1/job-postings")
                .cookie(admin.clone())
                .set_json(body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }

        let all: Vec<JobPosting> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/job-postings")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].title, "Active");
        assert_eq!(all[1].title, "Inactive");

        let active: Vec<JobPosting> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/job-postings?active=true")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].title, "Active");

        let inactive: Vec<JobPosting> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/job-postings?active=false")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(inactive.len(), 1);
        assert_eq!(inactive[0].title, "Inactive");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_rejects_invalid_input() {
        let (app, db, admin) = admin_app!();
        seed_job_posting!(app, admin);

        for (field, value) in [
            ("title", json!("")),
            ("title", json!("x".repeat(121))),
            ("company", json!("  ")),
            ("employment_type", json!(" ")),
            ("apply_url", json!("")),
            ("apply_url", json!("ftp://example.com")),
            ("author_id", json!(999)),
        ] {
            let mut body = job_posting_json("Engineer");
            body[field] = value;
            let req = TestRequest::post()
                .uri("/api/v1/job-postings")
                .cookie(admin.clone())
                .set_json(&body)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "{field}");
        }

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/1")
            .cookie(admin.clone())
            .set_json(json!({ "apply_url": "example.com" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/1")
            .cookie(admin.clone())
            .set_json(json!({ "title": "x".repeat(121) }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/1")
            .cookie(admin.clone())
            .set_json(json!({ "author_id": 999 }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req = TestRequest::patch()
            .uri("/api/v1/job-postings/999")
            .cookie(admin)
            .set_json(json!({ "title": "Ghost" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn job_posting_description_and_location_are_optional() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);

        let req = TestRequest::post()
            .uri("/api/v1/job-postings")
            .cookie(admin.clone())
            .set_json(json!({
                "title": "Minimal posting",
                "company": "Acme",
                "employment_type": "Contract",
                "apply_url": "https://example.com/jobs/2",
                "author_id": 1,
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created: JobPosting = test::read_body_json(resp).await;
        assert_eq!(created.description, "");
        assert_eq!(created.location, "");

        let mut body = job_posting_json("Blank optional fields");
        body["description"] = json!("");
        body["location"] = json!("  ");
        let req = TestRequest::post()
            .uri("/api/v1/job-postings")
            .cookie(admin)
            .set_json(body)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created: JobPosting = test::read_body_json(resp).await;
        assert_eq!(created.description, "");
        assert_eq!(created.location, "");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn alumni_delete_with_job_postings_fails_until_they_go() {
        let (app, db, admin) = admin_app!();
        seed_job_posting!(app, admin);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/alumni/1")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/job-postings/1")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let resp = test::call_service(
            &app,
            TestRequest::delete()
                .uri("/api/v1/alumni/1")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        cleanup(&db).await;
    }

    async fn body_text<B: actix_web::body::MessageBody>(resp: ServiceResponse<B>) -> String {
        String::from_utf8(test::read_body(resp).await.to_vec()).unwrap()
    }

    #[actix_web::test]
    async fn announcements_page_lists_and_escapes_html() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);
        let req = TestRequest::post()
            .uri("/api/v1/announcements")
            .cookie(admin.clone())
            .set_json(json!({ "title": "<b>Reunion</b>", "body": "Save the date", "author_id": 1 }))
            .to_request();
        assert_eq!(
            test::call_service(&app, req).await.status(),
            StatusCode::CREATED
        );

        let resp = test::call_service(
            &app,
            TestRequest::get()
                .uri("/announcements")
                .cookie(admin)
                .to_request(),
        )
        .await;
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
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);
        let req = announcement_form_post("/announcements", "title=Reunion&body=Save+the+date&author_id=1")
            .cookie(admin.clone())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/announcements");

        let announcement: Announcement = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/announcements/1")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(announcement.title, "Reunion");
        assert_eq!(announcement.body, "Save the date");

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_with_invalid_input_shows_the_error() {
        let (app, db, admin) = admin_app!();
        seed_alumni!(app, admin);
        let req = announcement_form_post(
            "/announcements",
            "title=&body=Save+the+date&author_id=1",
        )
        .cookie(admin)
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
        let (app, db, admin) = admin_app!();
        let req = announcement_form_post(
            "/announcements",
            "title=Reunion&body=Save+the+date&author_id=999",
        )
        .cookie(admin)
        .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_text(resp).await;
        assert!(body.contains("unknown author_id"));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcements_index_page_links_each_row_to_show_edit_and_delete() {
        let (app, db, admin) = admin_app!();
        seed_announcement!(app, admin);

        let resp = test::call_service(
            &app,
            TestRequest::get()
                .uri("/announcements")
                .cookie(admin)
                .to_request(),
        )
        .await;
        let body = body_text(resp).await;
        assert!(body.contains(r#"href="/announcements/1""#));
        assert!(body.contains(r#"href="/announcements/1/edit""#));
        assert!(body.contains(r#"action="/announcements/1/delete""#));

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_show_and_edit_pages_render_the_announcement() {
        let (app, db, admin) = admin_app!();
        seed_announcement!(app, admin);

        let resp = test::call_service(
            &app,
            TestRequest::get()
                .uri("/announcements/1")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_text(resp).await;
        assert!(body.contains("Save the date"));
        assert!(body.contains(r#"href="/announcements/1/edit""#));

        let resp = test::call_service(
            &app,
            TestRequest::get()
                .uri("/announcements/1/edit")
                .cookie(admin)
                .to_request(),
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
        let (app, db, admin) = admin_app!();
        seed_announcement!(app, admin);

        let resp = test::call_service(
            &app,
            announcement_form_post(
                "/announcements/1",
                "title=Picnic&body=Bring+food&author_id=1",
            )
            .cookie(admin.clone())
            .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/announcements/1");

        let announcement: Announcement = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/announcements/1")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(announcement.title, "Picnic");
        assert_eq!(announcement.author_id, 1);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn announcement_form_post_update_with_invalid_input_shows_the_edit_page_with_error() {
        let (app, db, admin) = admin_app!();
        seed_announcement!(app, admin);

        let resp = test::call_service(
            &app,
            announcement_form_post("/announcements/1", "title=&body=Bring+food&author_id=1")
                .cookie(admin)
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
        let (app, db, admin) = admin_app!();
        seed_announcement!(app, admin);

        let resp = test::call_service(
            &app,
            announcement_form_post("/announcements/1/delete", "")
                .cookie(admin.clone())
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(LOCATION).unwrap(), "/announcements");

        let resp = test::call_service(
            &app,
            TestRequest::get()
                .uri("/api/v1/announcements/1")
                .cookie(admin)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn missing_announcement_renders_an_html_404_page() {
        let (app, db, admin) = admin_app!();

        let requests = [
            TestRequest::get()
                .uri("/announcements/99")
                .cookie(admin.clone())
                .to_request(),
            TestRequest::get()
                .uri("/announcements/99/edit")
                .cookie(admin.clone())
                .to_request(),
            announcement_form_post("/announcements/99", "title=T&body=B&author_id=1")
                .cookie(admin.clone())
                .to_request(),
            announcement_form_post("/announcements/99/delete", "")
                .cookie(admin)
                .to_request(),
        ];
        for req in requests {
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let content_type = resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
            assert!(content_type.starts_with("text/html"));
        }

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn auth_register_creates_user_and_hides_password() {
        let (app, db) = test_app!();

        let req = TestRequest::post()
            .uri("/api/v1/auth/register")
            .set_json(json!({
                "first_name": "  Ada ",
                "last_name": " Lovelace ",
                "age": 24,
                "email": "Ada@Example.com",
                "academic_status": "grad",
                "password": "password123",
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let raw = body_text(resp).await;
        assert!(!raw.contains("password_hash"));
        let created: User = serde_json::from_str(&raw).unwrap();
        assert_eq!(created.id, 1);
        assert_eq!(created.first_name, "Ada");
        assert_eq!(created.last_name, "Lovelace");
        assert_eq!(created.full_name, "Ada Lovelace");
        assert_eq!(created.age, 24);
        assert_eq!(created.email, "ada@example.com");
        assert_eq!(created.roles, vec![Role::Unverified]);
        assert_eq!(created.requested_role, Some(Role::Grad));
        assert_eq!(created.verification_status, VerificationStatus::Pending);
        assert!(!created.email_verified);
        assert!(!created.phone_verified);
        assert!(created.is_active);

        let dup = TestRequest::post()
            .uri("/api/v1/auth/register")
            .set_json(json!({
                "first_name": "Ada",
                "last_name": "Clone",
                "age": 25,
                "email": "ada@example.com",
                "graduation_date": "2024-06-15",
                "password": "password123",
            }))
            .to_request();
        assert_eq!(
            test::call_service(&app, dup).await.status(),
            StatusCode::BAD_REQUEST
        );

        let admin = TestRequest::post()
            .uri("/api/v1/auth/register")
            .set_json(json!({
                "first_name": "Root",
                "last_name": "User",
                "age": 30,
                "email": "root@example.com",
                "academic_status": "admin",
                "password": "password123",
            }))
            .to_request();
        assert_eq!(
            test::call_service(&app, admin).await.status(),
            StatusCode::FORBIDDEN
        );

        let unauth_me =
            test::call_service(&app, TestRequest::get().uri("/api/v1/auth/me").to_request()).await;
        assert_eq!(unauth_me.status(), StatusCode::UNAUTHORIZED);

        let bad_login = TestRequest::post()
            .uri("/api/v1/auth/login")
            .set_json(json!({ "email": "ada@example.com", "password": "wrong-password" }))
            .to_request();
        assert_eq!(
            test::call_service(&app, bad_login).await.status(),
            StatusCode::UNAUTHORIZED
        );

        let login_req = TestRequest::post()
            .uri("/api/v1/auth/login")
            .set_json(json!({ "email": "ada@example.com", "password": "password123" }))
            .to_request();
        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::OK);
        let session_cookie = login_resp
            .response()
            .cookies()
            .find(|c| c.name() == "alumni_session")
            .expect("session cookie")
            .into_owned();
        let auth_session: AuthSession = test::read_body_json(login_resp).await;
        assert_eq!(auth_session.user.email, "ada@example.com");

        let me_resp = test::call_service(
            &app,
            TestRequest::get()
                .uri("/api/v1/auth/me")
                .cookie(session_cookie.clone())
                .to_request(),
        )
        .await;
        assert_eq!(me_resp.status(), StatusCode::OK);
        let me: User = test::read_body_json(me_resp).await;
        assert_eq!(me.id, created.id);

        let logout_resp = test::call_service(
            &app,
            TestRequest::post()
                .uri("/api/v1/auth/logout")
                .cookie(session_cookie.clone())
                .to_request(),
        )
        .await;
        assert_eq!(logout_resp.status(), StatusCode::NO_CONTENT);

        let revoked_me = test::call_service(
            &app,
            TestRequest::get()
                .uri("/api/v1/auth/me")
                .cookie(session_cookie)
                .to_request(),
        )
        .await;
        assert_eq!(revoked_me.status(), StatusCode::UNAUTHORIZED);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn auth_middleware_enforces_401_and_403_across_roles() {
        let (app, db) = test_app!();

        assert_eq!(
            test::call_service(&app, TestRequest::get().uri("/api/v1/alumni").to_request())
                .await
                .status(),
            StatusCode::UNAUTHORIZED
        );

        let (_, unverified) = session_for(&db, "new@example.com", Role::Unverified).await;
        assert_eq!(
            test::call_service(
                &app,
                TestRequest::get()
                    .uri("/api/v1/alumni")
                    .cookie(unverified)
                    .to_request(),
            )
            .await
            .status(),
            StatusCode::FORBIDDEN
        );

        let (alumni_id, alumnus) = session_for(&db, "alum@example.com", Role::Alumni).await;
        assert_eq!(
            test::call_service(
                &app,
                TestRequest::get()
                    .uri("/api/v1/alumni")
                    .cookie(alumnus.clone())
                    .to_request(),
            )
            .await
            .status(),
            StatusCode::OK
        );
        assert_eq!(
            test::call_service(
                &app,
                TestRequest::post()
                    .uri("/api/v1/announcements")
                    .cookie(alumnus.clone())
                    .set_json(json!({ "title": "Nope", "body": "Denied", "author_id": alumni_id }))
                    .to_request(),
            )
            .await
            .status(),
            StatusCode::FORBIDDEN
        );

        let post_req = TestRequest::post()
            .uri("/api/v1/job-postings")
            .cookie(alumnus)
            .set_json(json!({
                "title": "Rust Role",
                "company": "Acme",
                "employment_type": "Full-time",
                "apply_url": "https://example.com/apply",
                "author_id": alumni_id,
            }))
            .to_request();
        assert_eq!(
            test::call_service(&app, post_req).await.status(),
            StatusCode::CREATED
        );

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn admin_endpoints_require_admin_and_verify_or_update_users() {
        let (app, db) = test_app!();
        let (target_id, target_cookie) = session_for(&db, "student@example.com", Role::Unverified).await;
        let (_, alumni_cookie) = session_for(&db, "alum@example.com", Role::Alumni).await;
        let (admin_id, admin_cookie) = session_for(&db, "admin@example.com", Role::Admin).await;

        assert_eq!(
            test::call_service(
                &app,
                TestRequest::get()
                    .uri("/api/v1/admin/users")
                    .cookie(alumni_cookie)
                    .to_request(),
            )
            .await
            .status(),
            StatusCode::FORBIDDEN
        );

        let users: Vec<User> = test::call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri("/api/v1/admin/users")
                .cookie(admin_cookie.clone())
                .to_request(),
        )
        .await;
        assert_eq!(users.len(), 3);

        let verify_resp = test::call_service(
            &app,
            TestRequest::post()
                .uri(&format!("/api/v1/admin/users/{target_id}/verify"))
                .cookie(admin_cookie.clone())
                .set_json(json!({
                    "verification_status": "verified",
                    "roles": ["grad"],
                    "verification_method": "government_papers",
                }))
                .to_request(),
        )
        .await;
        assert_eq!(verify_resp.status(), StatusCode::OK);
        let verified: User = test::read_body_json(verify_resp).await;
        assert_eq!(verified.roles, vec![Role::Grad]);
        assert_eq!(verified.verification_status, VerificationStatus::Verified);

        let revoked_target = test::call_service(
            &app,
            TestRequest::get()
                .uri("/api/v1/auth/me")
                .cookie(target_cookie)
                .to_request(),
        )
        .await;
        assert_eq!(revoked_target.status(), StatusCode::UNAUTHORIZED);

        let self_disable = test::call_service(
            &app,
            TestRequest::patch()
                .uri(&format!("/api/v1/admin/users/{admin_id}"))
                .cookie(admin_cookie)
                .set_json(json!({ "is_active": false }))
                .to_request(),
        )
        .await;
        assert_eq!(self_disable.status(), StatusCode::BAD_REQUEST);

        cleanup(&db).await;
    }

    #[actix_web::test]
    async fn auth_update_me_updates_profile_and_password() {
        let (app, db) = test_app!();
        let reg = test::call_service(
            &app,
            TestRequest::post()
                .uri("/api/v1/auth/register")
                .set_json(json!({
                    "first_name": "Grace",
                    "last_name": "Hopper",
                    "age": 35,
                    "email": "grace@example.edu",
                    "academic_status": "phd",
                    "password": "initial-pass",
                }))
                .to_request(),
        )
        .await;
        assert_eq!(reg.status(), StatusCode::CREATED);

        let login = test::call_service(
            &app,
            TestRequest::post()
                .uri("/api/v1/auth/login")
                .set_json(json!({
                    "email": "grace@example.edu",
                    "password": "initial-pass",
                }))
                .to_request(),
        )
        .await;
        let cookie = login
            .response()
            .cookies()
            .find(|c| c.name() == "alumni_session")
            .expect("session cookie")
            .into_owned();

        let wrong_pw = test::call_service(
            &app,
            TestRequest::patch()
                .uri("/api/v1/auth/me")
                .cookie(cookie.clone())
                .set_json(json!({
                    "current_password": "wrong-pass",
                    "new_password": "updated-pass",
                }))
                .to_request(),
        )
        .await;
        assert_eq!(wrong_pw.status(), StatusCode::UNAUTHORIZED);

        let updated: User = test::call_and_read_body_json(
            &app,
            TestRequest::patch()
                .uri("/api/v1/auth/me")
                .cookie(cookie)
                .set_json(json!({
                    "first_name": "Rear Adm. Grace",
                    "phone_number": "+905551234567",
                    "current_password": "initial-pass",
                    "new_password": "updated-pass",
                }))
                .to_request(),
        )
        .await;
        assert_eq!(updated.full_name, "Rear Adm. Grace Hopper");
        assert_eq!(updated.phone_number.as_deref(), Some("+905551234567"));

        let relogin = test::call_service(
            &app,
            TestRequest::post()
                .uri("/api/v1/auth/login")
                .set_json(json!({
                    "email": "grace@example.edu",
                    "password": "updated-pass",
                }))
                .to_request(),
        )
        .await;
        assert_eq!(relogin.status(), StatusCode::OK);

        cleanup(&db).await;
    }
}
