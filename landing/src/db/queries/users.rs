use super::{models::DbUser, Result};
use sqlx::{PgPool, postgres::PgQueryResult};

/// User-related queries
pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str
) -> Result<DbUser> {
    sqlx::query_as!(
        DbUser,
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING *
        "#,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

/// Gets user by email with error handling
pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<DbUser> {
    sqlx::query_as!(
        DbUser,
        "SELECT * FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => DbError::NotFound,
        _ => e.into()
    })
}