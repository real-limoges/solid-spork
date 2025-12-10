// stubs for the handlers
use axum::{Json, http::StatusCode};
use crate::api::dtos::{DoSomethingRequest, DoSomethingResponse};


pub async fn root_handler() -> &'static str {
    "Server is Running ..."
}
pub async fn do_something_handler(
    Json(payload): Json<DoSomethingRequest>
) -> Result<(StatusCode, Json<DoSomethingResponse>), StatusCode> {
    // placeholder logic
    let everything_is_ok = true;

    if !everything_is_ok {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    else {
        let res = DoSomethingResponse { message: format!("Hello, {}! You are id: {}", payload.name, payload.id) };
        Ok((StatusCode::OK, Json(res)))
    }
}