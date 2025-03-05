use std::sync::Arc;

use r2d2::{self, Pool};
use redis::{Client, RedisError};

use super::env::Env;

pub async fn get_connection(env_variables: &Env) -> Result<Arc<Pool<Client>>, RedisError> {
    let client = redis::Client::open(env_variables.redis_url.to_string())?;

    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(client)
        .expect("Failed to create Redis connection pool");

    Ok(Arc::new(pool))
}
