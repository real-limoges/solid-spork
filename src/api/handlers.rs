// stubs for the handlers
use crate::api::dtos::{
    ModelARequest, ModelAResponse, ModelBRequest, ModelBResponse,
};
use axum::{Json, http::StatusCode};

pub async fn root_handler() -> &'static str {
    "Server is Running ..."
}

pub async fn post_model_a(
    Json(payload): Json<ModelARequest>,
) -> Result<(StatusCode, Json<ModelAResponse>), StatusCode> {
    let res = ModelAResponse { name: payload.name };
    Ok((StatusCode::OK, Json(res)))
}

pub async fn post_model_b(
    Json(payload): Json<ModelBRequest>,
) -> Result<(StatusCode, Json<ModelBResponse>), StatusCode> {
    let res = ModelBResponse { name: payload.name };
    Ok((StatusCode::OK, Json(res)))
}
