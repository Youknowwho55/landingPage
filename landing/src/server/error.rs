use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("User already exists")]
    UserExists,
    #[error("Database error")]
    DatabaseError,
    #[error("Invalid session")]
    InvalidSession,
    #[error("Password requirements not met")]
    PasswordRequirements,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Token Sorage Error")]
    TokenStorageFailed,
}