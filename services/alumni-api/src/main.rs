#![warn(unsafe_code)]

mod config;
mod controllers;
mod error;
mod handlers;
mod models;
mod routes;
mod user_store;
mod views;

use actix_web::{middleware::Logger, web, App, HttpServer};
use controllers::{api_user_controller, user_controller};
use mongodb::options::ClientOptions;
use mongodb::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "alumni-api",
        version = "0.1.0",
        description = "User pages (HTML) and the user REST API (JSON)."
    ),
    tags(
        (name = "UserController", description = "HTML pages under /users: list, create, show, edit, update, delete"),
        (name = "ApiUserController", description = "JSON user CRUD under /api/v1/users (alias: /api/users)"),
    ),
    paths(
        handlers::health,
        user_controller::index,
        user_controller::create,
        user_controller::show,
        user_controller::edit,
        user_controller::update,
        user_controller::delete,
        api_user_controller::create,
        api_user_controller::list,
        api_user_controller::get,
        api_user_controller::replace,
        api_user_controller::update,
        api_user_controller::delete,
    ),
    components(schemas(models::User, models::CreateUser, models::UpdateUser, handlers::Health))
)]
struct ApiDoc;

pub struct AppState {
    pub collection: mongodb::Collection<models::Alumni>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cfg = config::Config::from_env();

    let client_options = ClientOptions::parse(&cfg.mongo_uri)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let client = Client::with_options(client_options)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let collection = client.database(&cfg.mongo_db).collection("alumni");

    let state = web::Data::new(AppState { collection });
    let user_store = web::Data::new(user_store::UserStore::default());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(user_store.clone())
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
                    .service(handlers::list_alumni)
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
