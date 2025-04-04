use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::server::{AuthError, AuthProvider, User};

/// Core authentication context that manages user sessions and authentication state.
///
/// This struct provides thread-safe operations for:
/// - User login/logout
/// - Session management
/// - Current user state
///
/// # Type Parameters
/// - `AuthProvider`: Trait defining authentication operations
/// - `User`: Type representing authenticated users

pub struct AuthContext {
    /// The authentication provider implementation (e.g., PostgreSQL, OAuth)
    auth_provider: Arc<dyn AuthProvider + Send + Sync>,
    /// The currently authenticated user (protected by RwLock for thread safety)
    current_user: RwLock<Option<User>>,
}
impl std::fmt::Debug for AuthContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthContext")
            .field("auth_provider", &"<dyn AuthProvider>")
            .field("current_user", &self.current_user)
            .finish()
    }
}

impl AuthContext {
    /// Creates a new AuthContext with the specified authentication provider.
    ///
    /// # Arguments
    /// * `auth_provider` - The authentication provider implementation
    ///
    /// # Example
    /// ```rust
    /// let provider = Arc::new(PostgresAuth::new(pool));
    /// let auth_context = AuthContext::new(provider);
    /// ```
    pub fn new(auth_provider: Arc<dyn AuthProvider + Send + Sync>) -> Self {
        Self {
            auth_provider,
            current_user: RwLock::new(None),
        }
    }

    /// Authenticates a user and establishes a session.
    ///
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's password
    ///
    /// # Returns
    /// - `Ok(())` on successful authentication
    /// - `Err(AuthError)` on failure
    pub async fn login(&self, email: &str, password: &str) -> Result<(), AuthError> {
        let user = self.auth_provider.authenticate(email, password).await?;
        *self.current_user.write().await = Some(user);
        Ok(())
    }

    /// Registers a new user account.
    ///
    /// # Arguments
    /// * `email` - User's email address
    /// * `password` - User's password
    pub async fn register(&self, email: &str, password: &str) -> Result<(), AuthError> {
        self.auth_provider.register(email, password).await
    }

    /// Terminates the current user session.
    ///
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(AuthError)` if logout fails
    pub async fn logout(&self) -> Result<(), AuthError> {
        if let Some(user) = self.current_user.read().await.as_ref() {
            self.auth_provider.logout(&user.bearer_token).await?;
        }
        *self.current_user.write().await = None;
        Ok(())
    }

    /// Gets the currently authenticated user.
    ///
    /// # Returns
    /// - `Some(User)` if a user is logged in
    /// - `None` if no user is authenticated
    pub async fn current_user(&self) -> Option<User> {
        self.current_user.read().await.clone()
    }

    /// Validates and establishes a session from an existing token.
    ///
    /// # Arguments
    /// * `token` - Session token to validate
    ///
    /// # Returns
    /// - `Ok(User)` if the token is valid
    /// - `Err(AuthError)` if validation fails
    pub async fn validate_session(&self, token: &str) -> Result<User, AuthError> {
        let user = self.auth_provider.validate_session(token).await?;
        *self.current_user.write().await = Some(user.clone());
        Ok(user)
    }
}

// Removed PartialEq implementation as it's not meaningful for AuthContext
// Removed Default implementation as it requires a valid auth_provider




// Dioxus-compatible client wrapper
#[derive(Clone)]
pub struct AuthClient {
    inner: Arc<AuthContext>,
}

impl AuthClient {
    pub fn new(auth_provider: Arc<dyn AuthProvider + Send + Sync>) -> Self {
        Self {
            inner: Arc::new(AuthContext::new(auth_provider)),
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(), AuthError> {
        self.inner.login(email, password).await
    }

    pub async fn logout(&self) -> Result<(), AuthError> {
        self.inner.logout().await
    }

    pub async fn current_user(&self) -> Option<User> {
        self.inner.current_user().await
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<(), AuthError> {
        self.inner.register(email, password).await
    }

    pub async fn validate_session(&self, token: &str) -> Result<User, AuthError> {
        self.inner.validate_session(token).await
    }
}

// Dioxus hooks and provider
pub fn use_auth() -> AuthClient {
    use_context::<AuthClient>()
}





/// Context for handling protected route navigation.
/// 
/// Stores the originally requested route when authentication is required,
/// allowing redirection back after successful login.
#[derive(Default)]
pub struct GuardContext {
    next: Option<Routes>,
}

impl GuardContext {
    /// Sets the route to redirect to after authentication.
    pub fn set_next(next: Routes) {
        let mut guard = use_context::<Signal<GuardContext>>();
        guard.write().next = Some(next);
    }

    /// Redirects to the stored route or falls back to home.
    pub fn redirect_next_or_home() {
        let nav = navigator();
        let mut guard = use_context::<Signal<GuardContext>>();
        let next_maybe = guard.write().next.take();
        
        match next_maybe {
            Some(next) => { let _ = nav.push(next); },
            None => {
                match nav.push(Routes::Home {}) {
                    Some(_) => {},
                    None => log::error!("Navigation failed"),
                }
            },
        }
    }
}

