use super::user::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// DTOs for API communication
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSignupRequest {
    pub email: String,
    pub username: String,
    pub master_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub master_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
