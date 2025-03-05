use std::sync::Arc;

use crate::{
    dtos::{app_state::AppState, graphql_context::GraphQLContext},
    models::user_dtos::UserRedisSession,
    utils::error::{AppError, AppResult},
};
use async_graphql::Context;
use redis::Commands;

pub fn session_auth_middleware(ctx: &Context<'_>) -> AppResult<UserRedisSession> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let gql_ctx = ctx
        .data::<Arc<GraphQLContext>>()
        .map_err(|_| AppError::Internal("GraphQL Context is not passed".to_string()))?;

    let session_token = gql_ctx
        .session_token
        .as_ref()
        .ok_or(AppError::Authorization(
            "Session token is missing".to_string(),
        ))?;

    let mut redis_connection = app_state
        .redis_pool_manager
        .get()
        .map_err(|_| AppError::Internal("Failed to get redis connection from pool".to_string()))?;

    let redis_session_token = redis_connection
        .get::<String, String>(session_token.to_string())
        .map_err(|_| AppError::Authorization("Session token is invalid or expired".to_string()))?;

    if redis_session_token.len() == 0 {
        return Err(AppError::Authorization(
            "Session token is invalid or expired".to_string(),
        ));
    }

    // redis_session_token is the UserRedisSession struct serialized to string
    let user_redis_session = serde_json::from_str::<UserRedisSession>(&redis_session_token)
        .map_err(|_| {
            AppError::Internal("Failed to deserialize UserRedisSession from redis".to_string())
        })?;

    Ok(user_redis_session)
}

pub fn increment_session_expire(ctx: &Context<'_>) -> AppResult<()> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let env_variables = &app_state.env_variables;

    let gql_ctx = ctx
        .data::<Arc<GraphQLContext>>()
        .map_err(|_| AppError::Internal("GraphQL Context is not passed".to_string()))?;

    let session_token = gql_ctx
        .session_token
        .as_ref()
        .ok_or(AppError::Authorization(
            "Session token is missing".to_string(),
        ))?;

    let mut redis_connection = app_state
        .redis_pool_manager
        .get()
        .map_err(|_| AppError::Internal("Failed to get redis connection from pool".to_string()))?;

    redis_connection
        .expire::<String, usize>(
            session_token.to_string(),
            env_variables.session_expire_minutes * 60,
        )
        .map_err(|_| AppError::Internal("Failed to increment session expire".to_string()))?;

    Ok(())
}
