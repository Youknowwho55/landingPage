use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::server::{AuthError, AuthProvider, User};



// Core authentication context
#[derive(serde::Serialize, PartialEq)]

pub struct AuthContext {
    #[serde(skip_serializing)]
    auth_provider: Arc<dyn AuthProvider + Send + Sync>,
    #[serde(skip)]
    current_user: RwLock<Option<User>>,
}

impl AuthContext {
    pub fn new(auth_provider: Arc<dyn AuthProvider + Send + Sync>) -> Self {
        Self {
            auth_provider,
            current_user: RwLock::new(None),
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(), AuthError> {
        let user = self.auth_provider.authenticate(email, password).await?;
        *self.current_user.write().await = Some(user);
        Ok(())
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<(), AuthError> {
        self.auth_provider.register(email, password).await
    }

    pub async fn logout(&self) -> Result<(), AuthError> {
        if let Some(user) = self.current_user.read().await.as_ref() {
            self.auth_provider.logout(&user.bearer_token).await?;
        }
        *self.current_user.write().await = None;
        Ok(())
    }

    pub async fn current_user(&self) -> Option<User> {
        self.current_user.read().await.clone()
    }

    pub async fn validate_session(&self, token: &str) -> Result<User, AuthError> {
        let user = self.auth_provider.validate_session(token).await?;
        *self.current_user.write().await = Some(user.clone());
        Ok(user)
    }
}

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






