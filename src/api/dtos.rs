use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ModelARequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ModelAResponse {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelBRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ModelBResponse {
    pub name: String,
}
