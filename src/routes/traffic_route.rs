use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{
    controllers::traffic_controller::{
        get_traffic_data,
        get_traffic_data_by_zone,
    },
    AppState,
};

pub fn traffic_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_traffic_data))
        .route("/:zone", get(get_traffic_data_by_zone))
        .with_state(app_state)
}
