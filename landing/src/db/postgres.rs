use sqlx::{migrate::Migrator, PgPool};
use std::path::Path;

/// Runs database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), DbError> {
    Migrator::new(Path::new("./migrations"))
        .await?
        .run(pool)
        .await?;
    Ok(())
}