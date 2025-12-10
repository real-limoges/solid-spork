// stubs for the handlers
use axum::{Json, http::StatusCode};
use crate::api::dtos::{DoSomethingRequest, DoSomethingResponse};


pub async fn root_handler() -> &'static str {
    "Server is Running ..."
}
pub async fn do_something_handler(
    Json(payload): Json<DoSomethingRequest>
) -> (StatusCode, Json<DoSomethingResponse>) {
    // placeholder logic
    let res = DoSomethingResponse { message: format!("Hello, {}! You are id: {}", payload.name, payload.id) };

    (StatusCode::OK, Json(res))
}