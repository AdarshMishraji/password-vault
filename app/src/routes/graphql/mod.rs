mod mutation;
mod query;
mod service_schema;

use std::sync::Arc;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use axum::response::{Html, IntoResponse};

use mutation::Mutation;
use service_schema::ServiceSchema;

use crate::dtos::app_state::AppState;

pub async fn playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

pub fn schema(app_state: Arc<AppState>) -> ServiceSchema {
    Schema::build(query::Query, Mutation, EmptySubscription)
        .data(app_state.clone())
        .finish()
}
