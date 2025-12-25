/*
Handlers take the requests coming throuogh the router, and get them ready for the
IO Orchestration. They take the DTO and transform it into domain objects.
 */

use crate::api::dtos::{
    ModelARequest, ModelAResponse, ModelBRequest, ModelBResponse,
};
use crate::state::SharedAppState;
use axum::{Json, http::StatusCode, extract::State,};
use crate::api::DomainConversion;

pub async fn root_handler() -> &'static str {
    "Server is Running ..."
}

pub async fn post_model_a(
    State(_app_state): State<SharedAppState>,
    Json(payload): Json<ModelARequest>,
) -> Result<(StatusCode, Json<ModelAResponse>), StatusCode> {
    let _entity = payload.to_domain().map_err(|_| StatusCode::BAD_REQUEST)?;
    let res = ModelAResponse { name: payload.name };
    Ok((StatusCode::OK, Json(res)))
}

pub async fn post_model_b(
    State(_app_state): State<SharedAppState>,
    Json(payload): Json<ModelBRequest>,
) -> Result<(StatusCode, Json<ModelBResponse>), StatusCode> {
    let _entity = payload.to_domain().map_err(|_| StatusCode::BAD_REQUEST)?;
    let res = ModelBResponse { name: payload.name };
    Ok((StatusCode::OK, Json(res)))
}
