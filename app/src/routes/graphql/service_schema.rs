use async_graphql::{EmptySubscription, Schema};

use super::{mutation::Mutation, query::Query};

pub type ServiceSchema = Schema<Query, Mutation, EmptySubscription>;
