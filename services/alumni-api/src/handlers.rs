use actix_web::{get, web, web::Json};
use futures_util::TryStreamExt;
use mongodb::bson::doc;

use crate::error::ApiError;
use crate::models::Alumni;
use crate::AppState;

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
