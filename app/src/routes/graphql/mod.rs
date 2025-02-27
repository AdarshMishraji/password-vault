mod mutation;
mod query;
mod service_schema;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use axum::response::{Html, IntoResponse};

use mutation::Mutation;
use service_schema::ServiceSchema;

pub async fn playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

pub fn schema() -> ServiceSchema {
    Schema::build(query::Query, Mutation, EmptySubscription).finish()
}
