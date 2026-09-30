use actix_web::{get, web, web::Json, HttpResponse};
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::ApiError;
use crate::models::Alumni;
use crate::AppState;

#[derive(Serialize, ToSchema)]
pub struct Health {
    pub status: String,
}

/// Liveness check used for smoke tests.
#[utoipa::path(
    get,
    path = "/api/health",
    responses((status = 200, description = "Service is up", body = Health)),
)]
#[get("/api/health")]
pub async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".to_owned(),
    })
}

/// Redirects to the Swagger UI index (its mount has a trailing slash).
#[get("/api/swagger")]
pub async fn swagger_redirect() -> HttpResponse {
    HttpResponse::Found()
        .insert_header((actix_web::http::header::LOCATION, "/api/swagger/"))
        .finish()
}

#[get("/hello")]
pub async fn hello() -> Json<&'static str> {
    Json("hello, world")
}

#[get("/hello/{variable}")]
pub async fn hello_name(path: web::Path<String>) -> Json<String> {
    let variable = path.into_inner();
    Json(format!("hello, {variable}!"))
}

#[get("/sum/{num1}/{num2}")]
pub async fn sum(path: web::Path<(f64, f64)>) -> Json<f64> {
    let (num1, num2) = path.into_inner();
    Json(num1 + num2)
}

#[get("/alumni")]
pub async fn list_alumni(state: web::Data<AppState>) -> Result<Json<Vec<Alumni>>, ApiError> {
    let mut cursor = state.collection.find(doc! {}).await?;
    let mut alumni = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        alumni.push(doc);
    }
    Ok(Json(alumni))
}
