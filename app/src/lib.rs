mod configs;
mod constants;
mod dtos;
mod models;
mod routes;
mod services;
mod utils;

use std::{sync::Arc, time::Duration};

use axum::serve;
use configs::{database, env, redis};
use constants::art::ASCII_ART;
use dtos::app_state::AppState;
use tokio::time;
use utils::common::clr;

pub async fn init() {
    println!("Initializing App...");
    let env_variables = env::new();

    println!("Establishing Connection with Database...");
    let database_connection = match database::get_connection(&env_variables).await {
        Err(_) => panic!("Unable to Establish Connection with Database"),
        Ok(database_connection) => database_connection,
    };
    println!("Connection Established with Database 🚀");

    println!("Establishing Connection with Redis...");
    let redis_pool_manager = match redis::get_connection(&env_variables).await {
        Err(_) => panic!("Unable to Establish Connection with Redis"),
        Ok(redis_pool_manager) => redis_pool_manager,
    };
    println!("Connection Established with Redis 🚀");

    let app_state = Arc::new(AppState {
        database_connection,
        redis_pool_manager,
        env_variables,
    });

    let routes = routes::init_routes(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("App Initialized 🚀");

    time::sleep(Duration::from_millis(500)).await;

    clr();
    println!("{}", ASCII_ART);

    serve(listener, routes).await.unwrap();
}
