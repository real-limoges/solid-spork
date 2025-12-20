use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Framework JSON rejection: {0}")]
    AxumJson(#[from] JsonRejection),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Format parsing error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Redis database error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("SQL database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the full error detail
        error!(error = ?self, "AppError occurred");

        let (status, message) = match self {
            // Client-facing errors (4xx)
            AppError::Json(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation failed".to_string()),
            AppError::Parse(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),

            // Server-side errors (5xx) - mask details for security
            AppError::Config(_) | AppError::Redis(_) | AppError::Sqlx(_) | AppError::AxumJson(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}