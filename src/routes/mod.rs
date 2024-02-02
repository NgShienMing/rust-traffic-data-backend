use std::sync::Arc;

use axum::{
    Json,
    Router,
    routing::get,
    response::IntoResponse,
};

use crate::AppState;

mod traffic_route;

async fn entry_route_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres and Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}

fn entry_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(entry_route_handler))
        .with_state(app_state)
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .nest("/api", entry_route(app_state.clone()))
    .nest("/api/traffic", traffic_route::traffic_routes(app_state.clone()))
}