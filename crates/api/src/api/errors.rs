use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::domain::error::AppError;

pub enum ApiError {
    BadRequest { message: String },
    Unauthorized { message: String },
    Forbidden { message: String },
    NotFound { message: String },
    Conflict { message: String },
    InternalServerError,
}

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::NotFound { resource_type, .. } => ApiError::NotFound {
                message: format!("{resource_type} not found"),
            },
            AppError::Unauthorized { reason } => ApiError::Unauthorized {
                message: reason.to_string(),
            },
            AppError::Forbidden { reason } => ApiError::Forbidden {
                message: reason.to_string(),
            },
            AppError::Conflict { reason, .. } => ApiError::Conflict {
                message: reason.to_string(),
            },
            AppError::Validation { field, reason } => ApiError::BadRequest {
                message: format!("{field}: {reason}"),
            },
            AppError::Internal { message, source } => {
                tracing::error!("Internal error: {message}, source: {source:?}");
                ApiError::InternalServerError
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest { message } => (StatusCode::BAD_REQUEST, message),
            ApiError::Unauthorized { message } => (StatusCode::UNAUTHORIZED, message),
            ApiError::Forbidden { message } => (StatusCode::FORBIDDEN, message),
            ApiError::NotFound { message } => (StatusCode::NOT_FOUND, message),
            ApiError::Conflict { message } => (StatusCode::CONFLICT, message),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        };
        (status, serde_json::json!({ "error": message }).to_string()).into_response()
    }
}
