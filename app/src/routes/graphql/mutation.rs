use async_graphql::{Context, Object};
use validator::Validate;

use crate::{
    models::user_dtos::{
        UserLoginRequest, UserLoginResponse, UserSignupRequest, UserSignupResponse,
    },
    services::auth::{login, signup},
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
    ) -> AppResult<UserSignupResponse> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        signup(ctx, request).await
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        request: UserLoginRequest,
    ) -> AppResult<UserLoginResponse> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        login(ctx, request).await
    }
    // ********************* AUTH ************************//

    // ********************* PASSWORD ************************//
    async fn add_password(&self) -> String {
        "Add Password".to_string()
    }

    async fn delete_password(&self) -> String {
        "Delete Password".to_string()
    }

    async fn update_password(&self) -> String {
        "Update Password".to_string()
    }
    // ********************* PASSWORD ************************//
}
