use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod traffic_route;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .nest("/api/traffic", traffic_route::traffic_routes(app_state))
}