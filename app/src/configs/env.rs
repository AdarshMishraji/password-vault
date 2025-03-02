use std::sync::Arc;

use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Env {
    pub database_url: String,
    pub redis_url: String,
    pub recovery_keys_count: i32,
    pub session_expire_minutes: i32,
}

pub fn new() -> Arc<Env> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL is not set");

    let recovery_keys_count = std::env::var("RECOVERY_KEYS_COUNT")
        .expect("RECOVERY_KEYS_COUNT is not set")
        .parse::<i32>()
        .expect("RECOVERY_KEYS_COUNT is not a number");

    let session_expire_minutes = std::env::var("SESSION_EXPIRE_MINUTES")
        .expect("SESSION_EXPIRE_MINUTES is not set")
        .parse::<i32>()
        .expect("SESSION_EXPIRE_MINUTES is not a number");

    Arc::new(Env {
        database_url,
        redis_url,
        recovery_keys_count,
        session_expire_minutes,
    })
}
