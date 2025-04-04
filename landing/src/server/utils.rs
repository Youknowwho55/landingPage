use base64::engine::general_purpose;
use base64::Engine;
use bcrypt::verify;

use rand::{distr::Alphanumeric, Rng};
use web_sys::window;
use crate::server::error::AuthError;

/// Generates and stores session tokens securely
/// 
/// # Arguments
/// * `access_token` - The OAuth access token to store
/// * `refresh_token` - The OAuth refresh token to store
/// 
/// # Returns
/// `Result<String, AuthError>` - The generated session token or error
pub async fn generate_session_token() -> Result<String, AuthError> {
    let session_token: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    
    let encoded_token = general_purpose::URL_SAFE_NO_PAD.encode(session_token.as_bytes());
    
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            storage.set_item("session_token", &encoded_token)
                .map_err(|_| AuthError::TokenStorageFailed)?;
        }
    }
    
    Ok(encoded_token)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match verify(password, hash) {
        Ok(result) => result,
        Err(_) => false,
    }
}

pub fn meets_password_requirements(password: &str) -> bool {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    
    password.len() >= 8 && has_uppercase && has_lowercase && has_digit
}

pub fn is_valid_email(email: &str) -> bool {
    // Basic regex or more comprehensive validation
    !email.is_empty() && email.contains('@') && email.contains('.')
}