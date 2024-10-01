use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

use crate::{LOCKED_BALANCES, TOTAL_SUPPLY};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

pub async fn listen(addr: SocketAddr) -> () {
    let app = Router::new()
        .route("/", get(|| async { StatusCode::OK }))
        .route("/v1/circulating-supply", get(get_circulating_supply))
        .route("/v1/total-supply", get(get_total_supply));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CirculatingSupplyResponse {
    pub circulating_supply: u128,
}

pub async fn get_circulating_supply() -> impl IntoResponse {
    let mut locked_balance = 0;
    for (key, value) in LOCKED_BALANCES.lock().await.iter() {
        info!("{} {}", key, value);
        locked_balance += value;
    }

    let circulating_supply = TOTAL_SUPPLY.lock().await.clone() - locked_balance;
    let response = CirculatingSupplyResponse { circulating_supply };

    Response::new(serde_json::to_string(&response).unwrap())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalSupplyResponse {
    pub total_supply: u128,
}
pub async fn get_total_supply() -> impl IntoResponse {
    let total_supply = TOTAL_SUPPLY.lock().await.clone();
    let response = TotalSupplyResponse {
        total_supply,
    };

    Response::new(serde_json::to_string(&response).unwrap())
}
