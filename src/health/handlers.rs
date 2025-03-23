use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

// Define our API response data structures
#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
    pub status: String,
    pub code: u16,
}

// Handler for GET /api/health
pub async fn health_check() -> Json<ApiResponse> {
    let response = ApiResponse {
        message: "Hello from Axum!".into(),
        status: "success".into(),
        code: StatusCode::OK.as_u16(),
    };

    Json(response)
}
