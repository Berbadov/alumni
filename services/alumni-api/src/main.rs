#![warn(unsafe_code)]

mod alumni_store;
mod announcement_store;
mod config;
mod controllers;
mod error;
mod handlers;
mod job_posting_store;
mod models;
mod routes;
mod views;

use actix_web::{middleware::Logger, web, App, HttpServer};
use controllers::{
    announcement_controller, api_alumni_controller, api_announcement_controller,
    api_job_posting_controller,
};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{Client, IndexModel};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use alumni_store::AlumniStore;
use announcement_store::AnnouncementStore;
use job_posting_store::JobPostingStore;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "alumni-api",
        version = "0.1.0",
        description = "Alumni records (JSON), announcement pages (HTML), job postings (JSON), and their REST APIs."
    ),
    tags(
        (name = "ApiAlumniController", description = "JSON alumni CRUD under /api/v1/alumni"),
        (name = "AnnouncementController", description = "HTML pages under /announcements: list, create, show, edit, update, delete"),
        (name = "ApiAnnouncementController", description = "JSON announcement CRUD under /api/v1/announcements (alias: /api/announcements)"),
        (name = "ApiJobPostingController", description = "JSON job posting CRUD under /api/v1/job-postings"),
    ),
    paths(
        handlers::health,
        api_alumni_controller::create,
        api_alumni_controller::list,
        api_alumni_controller::get,
        api_alumni_controller::replace,
        api_alumni_controller::update,
        api_alumni_controller::delete,
        announcement_controller::index,
        announcement_controller::create,
        announcement_controller::show,
        announcement_controller::edit,
        announcement_controller::update,
        announcement_controller::delete,
        api_announcement_controller::create,
        api_announcement_controller::list,
        api_announcement_controller::get,
        api_announcement_controller::replace,
        api_announcement_controller::update,
        api_announcement_controller::delete,
        api_job_posting_controller::create,
        api_job_posting_controller::list,
        api_job_posting_controller::get,
        api_job_posting_controller::replace,
        api_job_posting_controller::update,
        api_job_posting_controller::delete,
    ),
    components(schemas(
        models::Alumni,
        models::CreateAlumni,
        models::UpdateAlumni,
        models::Announcement,
        models::CreateAnnouncement,
        models::UpdateAnnouncement,
        models::JobPosting,
        models::CreateJobPosting,
        models::UpdateJobPosting,
        handlers::Health
    ))
)]
struct ApiDoc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub alumni: AlumniStore,
    pub announcements: AnnouncementStore,
    pub job_postings: JobPostingStore,
}

fn io_error(err: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::other(err.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cfg = config::Config::from_env();

    let client_options = ClientOptions::parse(&cfg.mongo_uri)
        .await
        .map_err(io_error)?;
    let client = Client::with_options(client_options).map_err(io_error)?;
    let db = client.database(&cfg.mongo_db);

    db.collection::<mongodb::bson::Document>("alumni")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
        )
        .await
        .map_err(io_error)?;
    db.collection::<mongodb::bson::Document>("announcements")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "author_id": 1 })
                .build(),
        )
        .await
        .map_err(io_error)?;
    db.collection::<mongodb::bson::Document>("job_postings")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "author_id": 1 })
                .build(),
        )
        .await
        .map_err(io_error)?;
    db.collection::<mongodb::bson::Document>("job_postings")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "is_active": 1 })
                .build(),
        )
        .await
        .map_err(io_error)?;

    let state = web::Data::new(AppState {
        alumni: AlumniStore::new(&db),
        announcements: AnnouncementStore::new(&db),
        job_postings: JobPostingStore::new(&db),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/api/swagger/{_:.*}")
                    .url("/api/openapi.json", ApiDoc::openapi()),
            )
            .service(handlers::health)
            .service(handlers::swagger_redirect)
            .configure(routes::configure)
            .service(
                web::scope("/api/v1")
                    .service(handlers::hello)
                    .service(handlers::hello_name)
                    .service(handlers::sum),
            )
    })
    .bind(&cfg.bind_addr)?
    .workers(2)
    .shutdown_timeout(5)
    .run();

    let handle = server.handle();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            handle.stop(true).await;
        }
    });

    server.await
}
