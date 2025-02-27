use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// DTOs for API communication
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
