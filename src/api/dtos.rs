use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DoSomethingRequest {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct DoSomethingResponse {
    pub message: String,
}
