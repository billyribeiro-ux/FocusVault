use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Represents a user account.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new user account.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

/// Input for authenticating.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response returned after successful authentication.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: User,
}

/// API key for extension / desktop authentication.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Input for creating an API key.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateApiKey {
    pub name: String,
}

/// Response when creating an API key (includes the full key, shown only once).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyCreated {
    pub id: Uuid,
    pub name: String,
    pub key: String,
}
