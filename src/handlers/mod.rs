// handlers.rs
use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use tokio::sync::Mutex;

use crate::{Blockchain, Transaction};

type AppState = Arc<Mutex<Blockchain>>;

pub async fn get_chain(State(state): State<AppState>) -> impl IntoResponse {
    let blockchain = state.lock().await.clone();
    Json(blockchain)
}

pub async fn add_block(
    State(state): State<AppState>,
    Json(transactions): Json<Vec<Transaction>>,
) -> impl IntoResponse {
    let mut blockchain = state.lock().await;
    blockchain.add_block(transactions);
    Json(serde_json::json!({"message": "Block added successfully"}))
}
