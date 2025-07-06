use axum::{extract::Path, response::Json};
use serde_json::Value;
use reqwest::StatusCode;
use crate::utils::eth_provider::get_zentra_tx_data;
use crate::utils::load0::upload_to_load0;


pub async fn route_get() -> Json<Value> {
    Json(serde_json::json!({"status": "running"}))
}

pub async fn upload(Path(txid): Path<String>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match process_txid(&txid).await {
        Ok(load_hash) => Ok(Json(serde_json::json!({"load_hash": load_hash}))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))
        ))
    }
}

async fn process_txid(txid: &str) -> anyhow::Result<String> {
    let data = get_zentra_tx_data(txid).await?;
    let load_hash = upload_to_load0(data.1, "application/json").await?;
    Ok(load_hash)
}