use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),
    
    #[error("User already exists")]
    UserExists,
    
    #[error("User not found")]
    UserNotFound,
}