use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::health::handlers::health_check;

pub fn create_router() -> Router {
    Router::new().merge(health_routes())
    // .merge(some_other_routes())
    // .layer(TraceLayer::new_for_http())
}

fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
