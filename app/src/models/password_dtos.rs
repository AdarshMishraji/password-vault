use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// DTOs for API communication
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePasswordRequest {
    pub website_url: Option<String>,
    pub app_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub website_url: Option<String>,
    pub app_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordsPageResponse {
    pub passwords: Vec<PasswordResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
