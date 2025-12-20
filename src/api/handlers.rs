// stubs for the handlers
use crate::api::dtos::{
    DoSomethingRequest, DoSomethingResponse, ModelARequest, ModelAResponse, ModelBRequest,
    ModelBResponse,
};
use axum::{Json, http::StatusCode};

pub async fn root_handler() -> &'static str {
    "Server is Running ..."
}
pub async fn do_something_handler(
    Json(payload): Json<DoSomethingRequest>,
) -> Result<(StatusCode, Json<DoSomethingResponse>), StatusCode> {
    // placeholder logic
    let everything_is_ok = true;

    if !everything_is_ok {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        let res = DoSomethingResponse {
            message: format!("Hello, {}! You are id: {}", payload.name, payload.id),
        };
        Ok((StatusCode::OK, Json(res)))
    }
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
