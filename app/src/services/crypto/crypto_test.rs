#[cfg(test)]
mod test {
    use serde_json::{json, Value};

    use crate::{
        models::password,
        services::crypto::*,
        utils::error::{AppError, AppResult},
    };

    struct RecoveryCodeResponse {
        code_hash: String,
        encrypted_dek: String,
        recovery_code: String,
    }

    struct SignupResponse {
        recovery_codes: Vec<RecoveryCodeResponse>,
        master_password_hash: String,
        dek: [u8; 32],
        encrypted_dek: String,
    }

    fn signup() -> AppResult<SignupResponse> {
        let mut recovery_codes_data: Vec<RecoveryCodeResponse> = vec![];

        let master_password = "testing_password@123";

        let master_password_hash = hash_master_password(&master_password)?;

        let dek = generate_dek();

        let kek = derive_kek(&master_password)?;

        // Encrypt DEK with KEK
        let encrypted_dek = encrypt_dek(&dek, &kek)?;

        // Generate recovery codes
        let recovery_codes = generate_recovery_keys(5);

        for recovery_code in &recovery_codes {
            // Hash the recovery code for storage
            let code_hash = hash_recovery_code(recovery_code);

            // Encrypt DEK with recovery code as KEK
            let recovery_kek = derive_kek(recovery_code)?;
            let recovery_encrypted_dek = encrypt_dek(&dek, &recovery_kek)?;

            recovery_codes_data.push(RecoveryCodeResponse {
                code_hash,
                encrypted_dek: recovery_encrypted_dek,
                recovery_code: recovery_code.clone(),
            });
        }

        Ok(SignupResponse {
            recovery_codes: recovery_codes_data,
            master_password_hash,
            dek,
            encrypted_dek,
        })
    }

    #[test]
    fn test_recovery() -> AppResult<()> {
        let signup_data = signup()?;

        for recovery_code in signup_data.recovery_codes {
            let code_hash = hash_recovery_code(&recovery_code.recovery_code);

            if code_hash != recovery_code.code_hash {
                return Err(AppError::Crypto("Recovery code hash mismatch".to_string()));
            }

            let kek = derive_kek(&recovery_code.recovery_code)?;

            let dek = decrypt_dek(&recovery_code.encrypted_dek, &kek)?;

            if dek != signup_data.dek {
                return Err(AppError::Crypto("DEK mismatch".to_string()));
            }
        }

        Ok(())
    }

    #[test]
    fn test_password() -> AppResult<()> {
        let password = "testing_password@123";

        let hashed_master_password = hash_master_password(password)?;

        let is_same = verify_master_password(password, &hashed_master_password)?;

        if !is_same {
            return Err(AppError::Crypto("Password verification failed".to_string()));
        }

        Ok(())
    }
}
