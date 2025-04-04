use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

/// Alias for a PostgreSQL connection pool
pub type DbPool = Pool<Postgres>;

/// Creates a new PostgreSQL connection pool with recommended settings
///
/// # Arguments
/// * `database_url` - PostgreSQL connection string (e.g., "postgres://user:password@localhost/db")
///
/// # Returns
/// `Result<DbPool, sqlx::Error>` - Connection pool or error
///
/// # Example
/// ```rust
/// let pool = create_pool("postgres://user:password@localhost/db").await?;
/// ```
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)  // Adjust based on your application needs
        .min_connections(1)  // Maintain at least one connection
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(30))  // Close idle connections after 30s
        .max_lifetime(Duration::from_secs(1800)) // 30 minutes max connection lifetime
        .test_before_acquire(true)  // Test connections before use
        .connect(database_url)
        .await
}

/// Health check for the database
pub async fn check_health(pool: &DbPool) -> bool {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .is_ok()
}

