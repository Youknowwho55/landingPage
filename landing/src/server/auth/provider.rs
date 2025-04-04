//! Authentication provider trait definitions
//!
//! This module defines the core authentication trait that all providers must implement.

use crate::server::error::AuthError;
use crate::server::models::User;

use async_trait::async_trait;

/// Core authentication trait defining required operations
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Register a new user with email and password
    /// 
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's password
    /// 
    /// # Returns
    /// Result indicating success or specific authentication error
    async fn register(&self, email: &str, password: &str) -> Result<(), AuthError>;
    
    /// Authenticate a user with email and password
    /// 
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's password
    /// 
    /// # Returns
    /// On success, returns a User with session token. On failure, returns AuthError.
    async fn authenticate(&self, email: &str, password: &str) -> Result<User, AuthError>;
    
    /// Validate an existing session token
    /// 
    /// # Arguments
    /// * `token` - Session token to validate
    /// 
    /// # Returns
    /// User information if token is valid, AuthError otherwise
    async fn validate_session(&self, token: &str) -> Result<User, AuthError>;
    
    /// Invalidate a session token (logout)
    /// 
    /// # Arguments
    /// * `token` - Session token to invalidate
    async fn logout(&self, token: &str) -> Result<(), AuthError>;
}