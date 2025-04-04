pub mod auth {
    pub use super::auth::{
        AuthError,
        AuthProvider,
        PostgresAuth,
        new_pg_auth,
    };
}