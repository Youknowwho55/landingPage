// auth/mod.rs

// Module declarations
mod error;
mod provider;
mod postgres;  // Add this for PostgresAuth
mod models;
pub mod utils;
pub mod context;  // Keep pub if you want it accessible outside the auth module

// Re-exports
pub use error::AuthError;
pub use provider::AuthProvider;
pub use postgres::PostgresAuth;  // Now properly exported
pub use models::{User, Credentials}; 
pub use utils::generate_session_token;
pub use context::{AuthContext,use_auth};  // Only if you want this publicly exposed

// Optional: If you want to provide a prelude for easy importing
pub mod prelude {
    pub use super::{
        AuthError,
        AuthProvider,
        PostgresAuth,
        User,
        Credentials,
        generate_session_token,
        // Include AuthContext only if you want it in the prelude
    };
}

