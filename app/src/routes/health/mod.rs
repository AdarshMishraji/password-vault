use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::dtos::app_state::AppState;

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub async fn health(_: State<Arc<AppState>>) -> impl IntoResponse {
    let health = Health { healthy: true };
    (StatusCode::OK, Json(health))
}
