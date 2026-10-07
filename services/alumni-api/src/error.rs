use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use crate::user_store::UserError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("database error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<UserError> for ApiError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::Invalid(msg) => ApiError::BadRequest(msg.to_owned()),
            UserError::NotFound => ApiError::NotFound,
            UserError::Poisoned => ApiError::Internal(err.to_string()),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Mongo(_) | ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": self.to_string() }))
    }
}
