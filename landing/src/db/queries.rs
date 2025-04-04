use crate::db::models::{User, UserSession};
use sqlx::{PgPool, Error as SqlxError};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// User-related database queries
pub struct UserQueries;

/// Session-related database queries
pub struct SessionQueries;

impl UserQueries {
    /// Retrieves a user by their email address
    ///
    /// # Arguments
    /// * `pool` - PostgreSQL connection pool
    /// * `email` - Email address to search for
    ///
    /// # Returns
    /// - `Ok(Some(User))` if user found
    /// - `Ok(None)` if no user found
    /// - `Err(sqlx::Error)` on database error
    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, SqlxError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at 
            FROM users 
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    /// Creates a new user in the database
    ///
    /// # Arguments
    /// * `pool` - PostgreSQL connection pool
    /// * `email` - User's email address
    /// * `password_hash` - BCrypt-hashed password
    ///
    /// # Returns
    /// - `Ok(User)` with created user data
    /// - `Err(sqlx::Error)` on database error
    pub async fn create(
        pool: &PgPool,
        email: &str,
        password_hash: &str,
    ) -> Result<User, SqlxError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash) 
            VALUES ($1, $2) 
            RETURNING id, email, password_hash, created_at
            "#,
            email,
            password_hash
        )
        .fetch_one(pool)
        .await
    }
}

impl SessionQueries {
    /// Creates a new user session
    ///
    /// # Arguments
    /// * `pool` - PostgreSQL connection pool
    /// * `user_id` - ID of the user the session belongs to
    /// * `token` - Authentication token
    /// * `expires_at` - Session expiration timestamp
    ///
    /// # Returns
    /// - `Ok(UserSession)` with created session data
    /// - `Err(sqlx::Error)` on database error
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<UserSession, SqlxError> {
        sqlx::query_as!(
            UserSession,
            r#"
            INSERT INTO user_sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, expires_at, created_at
            "#,
            user_id,
            token,
            expires_at
        )
        .fetch_one(pool)
        .await
    }

    /// Deletes a session by its token (logout)
    pub async fn delete_by_token(pool: &PgPool, token: &str) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            DELETE FROM user_sessions
            WHERE token = $1
            "#,
            token
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }

    /// Validates and retrieves a session by token
    pub async fn validate_and_get(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<(User, UserSession)>, SqlxError> {
        sqlx::query_as!(
            (User, UserSession),
            r#"
            SELECT 
                u.id, u.email, u.password_hash, u.created_at,
                s.id, s.user_id, s.token, s.expires_at, s.created_at
            FROM user_sessions s
            JOIN users u ON s.user_id = u.id
            WHERE s.token = $1 AND s.expires_at > NOW()
            "#,
            token
        )
        .fetch_optional(pool)
        .await
    }
}