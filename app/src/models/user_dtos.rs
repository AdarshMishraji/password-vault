use super::user::Model;
use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Default, Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct UserSignupRequest {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub master_password: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, SimpleObject)]
pub struct UserSignupResponse {
    pub id: Uuid,
    pub recovery_keys: Vec<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct UserLoginRequest {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub master_password: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, SimpleObject)]
pub struct UserLoginResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoveryKeysResponse {
    pub recovery_keys: Vec<String>,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
