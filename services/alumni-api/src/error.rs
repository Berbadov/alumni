use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use crate::models::ModelError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<ModelError> for ApiError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::Invalid(msg) | ModelError::Blocked(msg) => {
                Self::BadRequest(msg.to_owned())
            }
            ModelError::NotFound => Self::NotFound,
            ModelError::Duplicate => Self::BadRequest("email already exists".to_owned()),
            ModelError::Internal(msg) => Self::Internal(msg),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": self.to_string() }))
    }
}
