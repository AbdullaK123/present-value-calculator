mod models;
mod utils;
mod tests;

use crate::models::*;
use crate::utils::*;
use axum::{Json, response::Result, Router, serve};
use axum::routing::{get, post};
use axum::http::StatusCode;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

async fn get_present_value(
    Json(cash_flows): Json<PresentValueRequest>,
) -> Result<(StatusCode, PresentValueResponse)> {
    let real_rate = real_rate(cash_flows.nominal_rate, cash_flows.inflation_rate);
    let result = present_value_stream(cash_flows.cash_flows, real_rate);
    Ok((StatusCode::OK, PresentValueResponse { result }))
}

async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}

pub fn app() -> Router {
    Router::new()
        .route("/present-value", post(get_present_value))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> Result<()>{

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = app();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();

    Ok(())
}