use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
};

mod models;
mod routes;
mod controllers;

async fn traffic_data_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // build our application with a single route
    let app = Router::new().route("/api/trafficdata", get(traffic_data_handler));
    println!("Server started successfully!");
    println!("Server running on port 3000");
    println!("Visit http://localhost:3000/api/trafficdata to see the response");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}