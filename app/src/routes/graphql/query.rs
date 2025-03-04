use async_graphql::{Context, Object};
use validator::Validate;

use crate::{
    dtos::response::GraphqlResponse,
    middlewares::auth::{increment_session_expire, session_auth_middleware},
    models::password_dtos::{
        GetPasswordRequest, GetPasswordsRequest, PasswordResponse, PasswordsPageResponse,
    },
    services::password::{get_password, get_passwords},
    utils::error::{AppError, AppResult},
};

pub struct Query;

#[Object]
impl Query {
    // ********************* AUTH ************************//
    // ********************* AUTH ************************//

    // ********************* PASSWORD ************************//
    async fn all_passwords(
        &self,
        ctx: &Context<'_>,
        request: GetPasswordsRequest,
    ) -> AppResult<GraphqlResponse<PasswordsPageResponse>> {
        let user_redis_session = session_auth_middleware(ctx).await?;

        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let response = get_passwords(ctx, &user_redis_session, request).await;

        increment_session_expire(ctx).await?;

        response
    }

    async fn get_password(
        &self,
        ctx: &Context<'_>,
        request: GetPasswordRequest,
    ) -> AppResult<GraphqlResponse<PasswordResponse>> {
        let user_redis_session = session_auth_middleware(ctx).await?;

        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let response = get_password(ctx, &user_redis_session, request).await;

        increment_session_expire(ctx).await?;

        response
    }
    // ********************* PASSWORD ************************//
}
