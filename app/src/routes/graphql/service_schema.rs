use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};

use crate::dtos::app_state::AppState;

use super::{
    mutation::Mutation,
    query::{self, Query},
};

pub type ServiceSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema(app_state: Arc<AppState>) -> ServiceSchema {
    Schema::build(query::Query, Mutation, EmptySubscription)
        .data(app_state.clone())
        .finish()
}
