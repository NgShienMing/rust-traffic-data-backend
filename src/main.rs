use std::sync::Arc;

use axum::http::{
    header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE},
    HeaderValue,
    Method,
};
use dotenv::dotenv;
use sqlx::{
    postgres::PgPoolOptions,
    Pool,
    Postgres
};
use tower_http::cors::CorsLayer;

mod models;
mod routes;
mod controllers;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to the database
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Build our application with a single route
    let app_state: Arc<AppState> = Arc::new(AppState{db: pool.clone()});
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = routes::create_router(app_state).layer(cors);

    println!("Server started successfully!");
    println!("Server running on port 3000");
    println!("Visit http://localhost:3000/api to see the response");

    // run our app with hyper, listening globally on port 3000
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}