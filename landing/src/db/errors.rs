use thiserror::Error;

/// Database operation errors
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection error")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("Migration failed")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Record not found")]
    NotFound,
}

/// Alias for database results
pub type Result<T> = std::result::Result<T, DbError>;