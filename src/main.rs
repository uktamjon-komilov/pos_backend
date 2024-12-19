mod utils;
mod api;

use axum::{ Router};
use axum::response::{IntoResponse};
use axum::routing::{get};
use serde::{Deserialize, Serialize};
use validator::{Validate};
use crate::api::auth;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/auth", auth::router())
        .route("/", get(health_check_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4269").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health_check_handler() -> &'static str {
    "OK"
}




