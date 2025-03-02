use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use argon2::{
    password_hash::{rand_core::RngCore, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::utils::error::{AppError, AppResult};
use base64::{engine::general_purpose, Engine as _};
// Constants for encryption
const NONCE_LENGTH: usize = 12;

/// Generate a random encryption key (DEK)
pub fn generate_dek() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Generate a salt for key derivation
fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

/// Hash the master password using Argon2
pub fn hash_master_password(password: &str) -> AppResult<String> {
    let salt_string = generate_salt();

    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| AppError::Crypto(e.to_string()))?
        .to_string();

    Ok(hashed_password)
}

/// Verify the master password against its hash
pub fn verify_master_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|e| AppError::Crypto(format!("Invalid password hash: {}", e.to_string())))?;

    let argon2_instance = Argon2::default();
    let result = argon2_instance
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(result)
}

/// Derive a key encryption key (KEK) from the master password
pub fn derive_kek(master_password: &str) -> AppResult<Key<Aes256Gcm>> {
    let result = Sha256::digest(master_password.as_bytes());

    let key = Key::<Aes256Gcm>::from_slice(&result).to_owned();
    Ok(key)
}

/// Encrypt the DEK with the KEK
pub fn encrypt_dek(dek: &[u8], kek: &Key<Aes256Gcm>) -> AppResult<String> {
    let cipher = Aes256Gcm::new(kek);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher_text = cipher
        .encrypt(&nonce, dek)
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    // Combine nonce and cipher_text and encode
    let mut encrypted = Vec::with_capacity(NONCE_LENGTH + cipher_text.len());
    encrypted.extend_from_slice(nonce.as_slice());
    encrypted.extend_from_slice(&cipher_text);

    Ok(general_purpose::STANDARD.encode(encrypted))
}

/// Decrypt the DEK with the KEK
pub fn decrypt_dek(encrypted_dek: &str, kek: &Key<Aes256Gcm>) -> AppResult<Vec<u8>> {
    let encrypted = general_purpose::STANDARD
        .decode(encrypted_dek)
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    if encrypted.len() < NONCE_LENGTH {
        return Err(AppError::Crypto("Invalid encrypted DEK".to_string()));
    }

    let nonce = Nonce::from_slice(&encrypted[..NONCE_LENGTH]);
    let cipher_text = &encrypted[NONCE_LENGTH..];

    let cipher = Aes256Gcm::new(kek);
    let plain_text = cipher
        .decrypt(nonce, cipher_text)
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    Ok(plain_text)
}

/// Encrypt a password with DEK
pub fn encrypt_password(password: &str, dek: &[u8]) -> AppResult<String> {
    let key = Key::<Aes256Gcm>::from_slice(dek);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher_text = cipher
        .encrypt(&nonce, password.as_bytes())
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    // Combine nonce and cipher_text and encode
    let mut encrypted = Vec::with_capacity(NONCE_LENGTH + cipher_text.len());
    encrypted.extend_from_slice(nonce.as_slice());
    encrypted.extend_from_slice(&cipher_text);

    Ok(general_purpose::STANDARD.encode(encrypted))
}

/// Decrypt a password with DEK
pub fn decrypt_password(encrypted_password: &str, dek: &[u8]) -> AppResult<String> {
    let encrypted = general_purpose::STANDARD
        .decode(encrypted_password)
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    if encrypted.len() < NONCE_LENGTH {
        return Err(AppError::Crypto("Invalid encrypted password".to_string()));
    }

    let nonce = Nonce::from_slice(&encrypted[..NONCE_LENGTH]);
    let cipher_text = &encrypted[NONCE_LENGTH..];

    let key = Key::<Aes256Gcm>::from_slice(dek);
    let cipher = Aes256Gcm::new(key);

    let plain_text = cipher
        .decrypt(nonce, cipher_text)
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    String::from_utf8(plain_text).map_err(|e| AppError::Crypto(e.to_string()))
}

/// Generate a random recovery key
pub fn generate_recovery_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    general_purpose::URL_SAFE.encode(key)
}

/// Hash a recovery code
pub fn hash_recovery_code(code: &str) -> String {
    let hash = Sha256::digest(code.as_bytes());
    format!("{:x}", hash)
}

/// Generate a secure token for sessions
pub fn generate_session_token() -> String {
    format!(
        "{}-{}",
        uuid::Uuid::new_v4(),
        general_purpose::URL_SAFE.encode(rand::rng().random::<[u8; 16]>())
    )
}

/// format multiple recovery codes
pub fn generate_recovery_keys(count: i32) -> Vec<String> {
    (0..count).map(|_| generate_recovery_key()).collect()
}
