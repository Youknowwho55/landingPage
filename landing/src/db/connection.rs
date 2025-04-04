use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use crate::db::errors::DbError;
/// Database connection pool type
pub type DbPool = PgPool;

/// Creates a new connection pool with default settings
///
/// # Example
/// ```rust
/// let pool = create_pool("postgres://user:pass@localhost/db").await?;
/// ```
pub async fn create_pool(database_url: &str) -> Result<DbPool, DbError> {
    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
        .map_err(Into::into)
}