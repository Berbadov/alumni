#![warn(unsafe_code)]

mod config;
mod error;
mod handlers;
mod models;

use std::time::Duration;

use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client;

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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(web::scope("/api/v1").service(handlers::list_alumni))
    })
    .bind(&cfg.bind_addr)?
    .workers(2)
    .shutdown_timeout(Duration::from_secs(5));

    let handle = server.handle();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            handle.stop(true).await;
        }
    });

    server.run().await
}
