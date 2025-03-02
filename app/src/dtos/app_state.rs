use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::configs::env::Env;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database_connection: Arc<DatabaseConnection>,
    pub env_variables: Arc<Env>,
}
