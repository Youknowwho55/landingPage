use base64::engine::general_purpose;
use base64::Engine;
use bcrypt::verify;
use rand::Rng;

pub fn generate_session_token() -> String {
    let random_bytes: Vec<u8> = rand::thread_rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(32)
        .collect();
    
    general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
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