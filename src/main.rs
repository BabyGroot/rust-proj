use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod health;
mod router;

// Define our API response data structures
#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
    code: u16,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = router::create_router();

    // Define the address
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    tracing::info!("Server running at http://localhost:{}", addr.port());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    // Start the server
    axum::serve(listener, app).await.unwrap();
}
