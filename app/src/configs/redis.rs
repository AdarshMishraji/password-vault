use std::sync::Arc;

use r2d2_redis::{
    r2d2::{self, Pool},
    redis::RedisError,
    RedisConnectionManager,
};

use super::env::Env;

pub async fn get_connection(
    env_variables: &Env,
) -> Result<Arc<Pool<RedisConnectionManager>>, RedisError> {
    let manager = RedisConnectionManager::new(env_variables.redis_url.clone())
        .expect("Failed to create Redis connection manager");

    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create Redis connection pool");

    Ok(Arc::new(pool))
}
