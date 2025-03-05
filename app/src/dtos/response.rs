use async_graphql::{OutputType, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::models::{
    password_dtos::{PasswordResponse, PasswordsPageResponse},
    user_dtos::{RecoveryKeyResponse, UserSignupResponse},
};

#[derive(Clone, Default, Debug, Serialize, Deserialize, SimpleObject)]
pub struct GraphqlGenericResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(
    name = "GraphqlResponse_UserSignupResponse",
    params(UserSignupResponse)
))]
#[graphql(concrete(
    name = "GraphqlResponse_PasswordsPageResponse",
    params(PasswordsPageResponse)
))]
#[graphql(concrete(name = "GraphqlResponse_PasswordResponse", params(PasswordResponse)))]
#[graphql(concrete(
    name = "GraphqlResponse_RecoveryKeyResponse",
    params(RecoveryKeyResponse)
))]
pub struct GraphqlResponse<T>
where
    T: Send + Sync + OutputType,
{
    pub success: bool,
    pub message: String,
    pub data: T,
}
