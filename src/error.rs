use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::rejection::JsonRejection
};
use serde_json::json;
use std::fmt;
use std::num::ParseIntError;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Config(String),
    AxumJson(JsonRejection),
    Json(serde_json::Error),
    Parse(ParseIntError),
    Redis(redis::RedisError),
    Sqlx(sqlx::Error),
    Validation(validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Config(e) => {
                error!("Configuration error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Server configuration error: {}", e),
                )
            },
            AppError::AxumJson(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid Json Error: {}", e),
            ),
            AppError::Json(e) => (
                StatusCode::BAD_REQUEST,
                format!("JSON error: {}", e)
            ),
            AppError::Parse(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Parse error: {}", e),
            ),
            AppError::Redis(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Redis error: {}", e)
            ),
            AppError::Sqlx(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("SQLX Error: {}", e)
            ),
            AppError::Validation(e) => (
                StatusCode::BAD_REQUEST,
                format!("Validation error: {}", e)
            ),
        };
        let body = Json(json!({ "error_message": error_message }));
        (status, body).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Config Error: {}", e),
            AppError::Json(e) => write!(f, "Json Error: {}", e),
            AppError::Parse(e) => write!(f, "Parse Error: {}", e),
            AppError::Redis(e) => write!(f, "Redis Error: {}", e),
            AppError::Sqlx(e) => write!(f, "SQLx Error: {}", e),
            &AppError::AxumJson(_) | &AppError::Validation(_) => todo!(),
        }
    }
}

impl std::error::Error for AppError {}

impl From<JsonRejection> for AppError {
    fn from(e: JsonRejection) -> Self {
        AppError::AxumJson(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Json(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}
impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        AppError::Redis(e)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Sqlx(e)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        AppError::Validation(e)
    }
}