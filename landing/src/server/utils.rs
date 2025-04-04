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
#[cfg(target_arch = "wasm32")]
pub async fn generate_session_token(access_token: &str, refresh_token: &str) -> Result<String, AuthError> {
    use web_sys::window;

    let session_token = generate_random_token(); // Your token generation logic
    
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            storage.set_item("access_token", access_token)
                .map_err(|_| AuthError::TokenStorageFailed)?;
            storage.set_item("refresh_token", refresh_token)
                .map_err(|_| AuthError::TokenStorageFailed)?;
            storage.set_item("session_token", &session_token)
                .map_err(|_| AuthError::TokenStorageFailed)?;
        }
    }
    Ok(session_token)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn generate_session_token(access_token: &str, refresh_token: &str) -> Result<String, AuthError> {
    // For desktop/native targets, use a different storage method
    // Example: File system, SQLite, or system keychain
    let session_token = generate_random_token();
    
    // Implement native storage here (example using `keyring` crate)
    let entry = keyring::Entry::new("your_app", "access_token")
        .map_err(|_| AuthError::TokenStorageFailed)?;
    entry.set_password(access_token)
        .map_err(|_| AuthError::TokenStorageFailed)?;
    
    Ok(session_token)
}

pub fn generate_random_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
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