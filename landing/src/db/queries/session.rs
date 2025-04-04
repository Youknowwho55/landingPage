use super::{models::UserSession, Result};
use sqlx::PgPool;

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    token: &str,
    expires_at: DateTime<Utc>
) -> Result<UserSession> {
    sqlx::query_as!(
        UserSession,
        r#"
        INSERT INTO user_sessions (user_id, token, expires_at)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        user_id,
        token,
        expires_at
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}