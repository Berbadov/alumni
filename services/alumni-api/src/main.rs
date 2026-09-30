#![warn(unsafe_code)]

mod config;
mod error;
mod handlers;
mod models;
mod users;

use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "alumni-api", version = "0.1.0"),
    paths(
        handlers::health,
        users::create_user,
        users::list_users,
        users::get_user,
        users::replace_user,
        users::update_user,
        users::delete_user,
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
    let user_store = web::Data::new(users::UserStore::default());

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
            .service(users::create_user)
            .service(users::list_users)
            .service(users::get_user)
            .service(users::replace_user)
            .service(users::update_user)
            .service(users::delete_user)
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
