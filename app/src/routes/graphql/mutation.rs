use async_graphql::{Context, Object};
use validator::Validate;

use crate::{
    dtos::response::{GraphqlGenericResponse, GraphqlResponse},
    middlewares::auth::{increment_session_expire, session_auth_middleware},
    models::{
        password_dtos::{AddPasswordRequest, DeletePasswordRequest, UpdatePasswordRequest},
        user_dtos::{UserLoginRequest, UserSignupRequest, UserSignupResponse},
    },
    services::{
        auth::{login, logout, signup},
        password::{add_password, delete_password, update_password},
    },
    utils::error::{AppError, AppResult},
};

pub struct Mutation;

#[Object]
impl Mutation {
    // ********************* AUTH ************************//
    async fn signup(
        &self,
        ctx: &Context<'_>,
        request: UserSignupRequest,
    ) -> AppResult<GraphqlResponse<UserSignupResponse>> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        signup(ctx, request).await
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        request: UserLoginRequest,
    ) -> AppResult<GraphqlGenericResponse> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        login(ctx, request).await
    }

    async fn logout(&self, ctx: &Context<'_>) -> AppResult<GraphqlGenericResponse> {
        let user_redis_session = session_auth_middleware(ctx)?;

        logout(ctx, &user_redis_session).await
    }
    // ********************* AUTH ************************//

    // ********************* PASSWORD ************************//
    async fn add_password(
        &self,
        ctx: &Context<'_>,
        request: AddPasswordRequest,
    ) -> AppResult<GraphqlGenericResponse> {
        let user_redis_session = session_auth_middleware(ctx)?;

        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let response = add_password(ctx, &user_redis_session, request).await;

        increment_session_expire(ctx)?;

        response
    }

    async fn update_password(
        &self,
        ctx: &Context<'_>,
        request: UpdatePasswordRequest,
    ) -> AppResult<GraphqlGenericResponse> {
        let user_redis_session = session_auth_middleware(ctx)?;

        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let response = update_password(ctx, &user_redis_session, request).await;

        increment_session_expire(ctx)?;

        response
    }

    async fn delete_password(
        &self,
        ctx: &Context<'_>,
        request: DeletePasswordRequest,
    ) -> AppResult<GraphqlGenericResponse> {
        let user_redis_session = session_auth_middleware(ctx)?;

        let response = delete_password(ctx, &user_redis_session, request).await;

        increment_session_expire(ctx)?;

        response
    }
    // ********************* PASSWORD ************************//
}
