 // User and DbUser models

 use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub bearer_token: String,
    pub token_expires_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}