use axum::{extract::Path, response::Json};
use serde_json::Value;


pub async fn route_get() -> Json<Value> {
    Json(serde_json::json!({"status": "running"}))
}
