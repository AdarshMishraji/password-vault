mod graphql;
mod health;

use std::sync::Arc;

use async_graphql_axum::GraphQL;
use axum::{routing::get, Router};

use health::health;

use crate::dtos::app_state::AppState;

pub fn init_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            get(graphql::playground).post_service(GraphQL::new(graphql::schema(app_state.clone()))),
        )
        .route("/health", get(health))
        .with_state(app_state.clone())
}
