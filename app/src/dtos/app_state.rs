use std::sync::Arc;

use r2d2_redis::{r2d2::Pool, RedisConnectionManager};
use sea_orm::DatabaseConnection;

use crate::configs::env::Env;

#[derive(Clone)]
pub struct AppState {
    pub database_connection: Arc<DatabaseConnection>,
    pub redis_pool_manager: Arc<Pool<RedisConnectionManager>>,
    pub env_variables: Arc<Env>,
}
