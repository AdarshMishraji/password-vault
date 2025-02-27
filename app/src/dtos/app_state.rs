use sea_orm::DatabaseConnection;

use crate::configs::env::Env;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database_connection: DatabaseConnection,
    pub env_variables: Env,
}
