use super::AuthProvider;
use crate::{
    db::DbPool,
    server::{
        error::AuthError,
        models::{User, DbUser},
        utils::{generate_session_token, is_valid_email, meets_password_requirements, verify_password},
    },
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};

/// PostgreSQL implementation of the AuthProvider trait
///
/// Handles all authentication operations against a PostgreSQL database including:
/// - User registration
/// - Authentication (login)
/// - Session validation
/// - Logout functionality
pub struct PostgresAuth {
    /// Database connection pool
    pool: DbPool,
}

impl PostgresAuth {
    /// Creates a new PostgresAuth instance with the given database pool
    ///
    /// # Arguments
    /// * `pool` - An established connection pool to a PostgreSQL database
    ///
    /// # Example
    /// ```rust
    /// let pool = create_db_pool().await;
    /// let auth_provider = PostgresAuth::new(pool);
    /// ```
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthProvider for PostgresAuth {
    /// Registers a new user with the system
    ///
    /// Performs validation checks before creating the user record:
    /// 1. Validates email format
    /// 2. Validates password meets complexity requirements
    /// 3. Checks for existing user with same email
    /// 4. Hashes password before storage
    async fn register(&self, email: &str, password: &str) -> Result<(), AuthError> {
        // Validate email format
        if !is_valid_email(email) {
            return Err(AuthError::InvalidEmail);
        }
        
        // Validate password meets requirements
        if !meets_password_requirements(password) {
            return Err(AuthError::PasswordRequirements);
        }

        let mut client = self.pool.acquire().await.map_err(|_| AuthError::DatabaseError)?;
        
        // Check if user already exists
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
        )
        .bind(email)
        .fetch_one(&mut *client)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

        if exists {
            return Err(AuthError::UserExists);
        }

        // Hash password for secure storage
        let hashed_password = hash(password, DEFAULT_COST)
            .map_err(|_| AuthError::DatabaseError)?;

        // Insert new user record
// Insert new user record
sqlx::query(
    "INSERT INTO users (email, password_hash) VALUES ($1, $2)"
)
.bind(email)
.bind(hashed_password)
.execute(&mut *client)
.await
.map_err(|_| AuthError::DatabaseError)?;

        Ok(())
    }

    /// Authenticates a user and creates a new session
    ///
    /// # Flow
    /// 1. Retrieves user record by email
    /// 2. Verifies password against stored hash
    /// 3. Generates new session token
    /// 4. Stores session in database with 30-day expiration
    async fn authenticate(&self, email: &str, password: &str) -> Result<User, AuthError> {
        let mut client = self.pool.acquire().await.map_err(|_| AuthError::DatabaseError)?;
        

        // Retrieve user from database
        let db_user: DbUser = match client.query_one(
            "SELECT id, email, password_hash FROM users WHERE email = $1",
            &[&email]
        ) {
            Ok(row) => DbUser {
                id: row.get(0),
                email: row.get(1),
                password_hash: row.get(2),
            },
            Err(_) => return Err(AuthError::AuthenticationFailed),
        };

        // Verify provided password against stored hash
        if !verify_password(password, &db_user.password_hash) {
            return Err(AuthError::AuthenticationFailed);
        }

        // Generate new session token
        let token = generate_session_token();
        let expires_at = Utc::now() + Duration::days(30);
        
        // Store session in database
        client.execute(
            "INSERT INTO user_sessions (user_id, token, expires_at) VALUES ($1, $2, $3)",
            &[&db_user.id, &token, &expires_at]
        )
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(User {
            id: db_user.id,
            email: db_user.email,
            bearer_token: token,
            token_expires_at: expires_at,
        })
    }

    /// Validates an existing session token
    ///
    /// Checks:
    /// - Token exists in database
    /// - Token hasn't expired
    /// - Associated user exists
    async fn validate_session(&self, token: &str) -> Result<User, AuthError> {
        let mut client = self.pool.lock().await;
        
        // Validate session token and get user info
        let row = client.query_one(
            "SELECT u.id, u.email, s.expires_at 
             FROM user_sessions s
             JOIN users u ON s.user_id = u.id
             WHERE s.token = $1 AND s.expires_at > NOW()",
            &[&token]
        )
        .map_err(|_| AuthError::InvalidSession)?;

        Ok(User {
            id: row.get(0),
            email: row.get(1),
            bearer_token: token.to_string(),
            token_expires_at: row.get(2),
        })
    }

    /// Invalidates a session token (logout)
    ///
    /// Simply deletes the session token from the database,
    /// preventing further use of that token for authentication.
    async fn logout(&self, token: &str) -> Result<(), AuthError> {
        let mut client = self.pool.lock().await;
        
        // Delete session token from database
        client.execute(
            "DELETE FROM user_sessions WHERE token = $1",
            &[&token]
        )
        .map_err(|_| AuthError::DatabaseError)?;

        Ok(())
    }
}