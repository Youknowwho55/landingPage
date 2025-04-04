use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database representation of a user
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Active user session record
#[derive(Debug, sqlx::FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Combined user profile data (for complex queries)
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserProfile {
    #[sqlx(flatten)]
    pub user: DbUser,
    pub session_count: i64,
}