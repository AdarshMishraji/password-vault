pub mod graphql;
mod health;

use std::sync::Arc;

use axum::{routing::get, Extension, Router};
pub use graphql::service_schema::schema;
use health::health;

use crate::dtos::app_state::AppState;

pub fn init_routes(app_state: Arc<AppState>) -> Router {
    let schema = schema(app_state.clone());
    Router::new()
        .route("/", get(graphql::playground).post(graphql::graphql_handler))
        .route("/health", get(health))
        .layer(Extension(schema))
        .with_state(app_state)
}
