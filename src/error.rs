use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::rejection::JsonRejection
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Config(String),
    Json(JsonRejection),
    Redis(redis::RedisError),
    Sqlx(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Config(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Config Error: {}", e)
            ),
            AppError::Json(e) => (
                StatusCode::BAD_REQUEST,
                format!("JSON error: {}", e)
            ),
            AppError::Redis(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Redis error: {}", e)
            ),
            AppError::Sqlx(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("SQLX Error: {}", e)
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
            AppError::Redis(e) => write!(f, "Redis Error: {}", e),
            AppError::Sqlx(e) => write!(f, "SQLx Error: {}", e),
        }
    }
}