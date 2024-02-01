use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{
    models::traffic_model::Traffic,
    AppState,
};

#[derive(Debug, serde::Deserialize)]
pub struct TrafficQueryParams {
    district: String,
    device: String,
}

pub async fn get_traffic_data(
    Query(query_params): Query<TrafficQueryParams>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("hello");
    let traffic_data = sqlx::query_as!(
        Traffic,
        "SELECT * FROM traffic_data WHERE district = $1 AND device = $2",
        query_params.district, query_params.device
    )
    .fetch_all(&data.db)
    .await;
    // let traffic_data = Traffic::get_traffic_data(query_params.district, query_params.device).await;
    match traffic_data {
        Ok(traffic_data) => Ok(Json(traffic_data)),
        Err(err) => {
            let json_response = json!({
                "status": "error",
                "message": format!("{}", err)
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)))
        }
    }
}

pub async fn get_traffic_data_by_zone(
    Path(zone): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let traffic_data = sqlx::query_as!(
        Traffic,
        "SELECT * FROM traffic_data WHERE zone = $1",
        zone
    )
    .fetch_all(&data.db)
    .await;
    match traffic_data {
        Ok(traffic_data) => Ok(Json(traffic_data)),
        Err(err) => {
            let json_response = json!({
                "status": "error",
                "message": format!("{}", err)
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)))
        }
    }
}
