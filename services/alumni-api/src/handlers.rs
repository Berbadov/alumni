use actix_web::{get, web, web::Json};
use futures_util::TryStreamExt;

use crate::error::ApiError;
use crate::models::Alumni;
use crate::AppState;

#[get("/alumni")]
pub async fn list_alumni(state: web::Data<AppState>) -> Result<Json<Vec<Alumni>>, ApiError> {
    let mut cursor = state.collection.find(None, None).await?;
    let mut alumni = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        alumni.push(doc);
    }
    Ok(Json(alumni))
}
