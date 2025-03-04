mod mutation;
mod query;
pub mod service_schema;

use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Extension,
    http::{HeaderMap, header},
    response::{Html, IntoResponse},
};
use service_schema::ServiceSchema;

use crate::dtos::graphql_context::GraphQLContext;

pub async fn playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn graphql_handler(
    schema: Extension<ServiceSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let cookie = headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());

    let session_token = cookie.and_then(|cookie| {
        let token = cookie
            .split(';')
            .find(|cookie| cookie.starts_with("session_token="))
            .map(|cookie| cookie.replace("session_token=", ""));
        token
    });

    let gql_ctx = Arc::new(GraphQLContext {
        session_token,
        headers: Some(headers),
    });

    schema.execute(req.into_inner().data(gql_ctx)).await.into()
}
