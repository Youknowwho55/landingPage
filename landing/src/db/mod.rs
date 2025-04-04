//! Database access layer
//!
//! Provides:
//! - Connection pooling
//! - Schema definitions
//! - Raw query operations
//! - Error handling

mod connection;
mod errors;
mod models;
mod postgres;
mod queries;

// Public interface
pub use connection::{create_pool, DbPool};
pub use errors::{DbError, Result};
pub use models::{DbUser, UserSession, UserProfile};
pub use postgres::run_migrations;
pub use queries::{session, users};

/// Re-export for convenience
pub use sqlx::postgres::PgPool;

/// Database result type with custom error handling
pub type Result<T> = std::result::Result<T, DbError>;