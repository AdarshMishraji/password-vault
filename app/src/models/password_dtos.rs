use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::validators::password::{
    validate_add_password_request, validate_get_password_request, validate_get_passwords_request,
    validate_update_password_request,
};

// DTOs for API communication
#[derive(Clone, Default, Debug, Serialize, Deserialize, InputObject, Validate)]
#[validate(schema(function = "validate_add_password_request"))]
pub struct AddPasswordRequest {
    pub website_url: Option<String>,
    pub app_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, InputObject, Validate)]
#[validate(schema(function = "validate_update_password_request"))]
pub struct UpdatePasswordRequest {
    pub id: Uuid,
    pub website_url: Option<String>,
    pub app_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, InputObject)]
pub struct DeletePasswordRequest {
    pub id: Uuid,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct PasswordResponse {
    pub id: Uuid,
    pub website_url: Option<String>,
    pub app_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct PasswordsPageResponse {
    pub passwords: Vec<PasswordResponse>,
    pub page: u64,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, InputObject, Validate)]
#[validate(schema(function = "validate_get_passwords_request"))]
pub struct GetPasswordsRequest {
    pub page: u64,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
#[validate(schema(function = "validate_get_password_request"))]
pub struct GetPasswordRequest {
    pub id: Option<Uuid>,
    pub website_url: Option<String>,
    pub app_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}
