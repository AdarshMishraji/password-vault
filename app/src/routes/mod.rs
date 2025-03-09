pub mod graphql;
mod health;

use std::sync::Arc;

use axum::{Extension, Router, routing::get};
pub use graphql::service_schema::schema;
use health::health;
use tower_http::trace::TraceLayer;

use crate::{dtos::app_state::AppState, middlewares::trace::tracer};

pub fn init_routes(app_state: Arc<AppState>) -> Router {
    let schema = schema(app_state.clone());

    tracing_subscriber::fmt::init();

    Router::new()
        .route("/", get(graphql::playground).post(graphql::graphql_handler))
        .route("/health", get(health))
        .layer(Extension(schema))
        .layer(tracer())
        .with_state(app_state)
}
