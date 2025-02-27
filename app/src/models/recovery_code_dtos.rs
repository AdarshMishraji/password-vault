use serde::{Deserialize, Serialize};

// DTOs for API communication
#[derive(Debug, Serialize, Deserialize)]
pub struct RecoveryRequest {
    pub email: String,
    pub recovery_code: String,
    pub new_master_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeMasterPasswordRequest {
    pub old_master_password: String,
    pub new_master_password: String,
}
