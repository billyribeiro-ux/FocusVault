use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProject {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub color: Option<Option<String>>,
    pub archived: Option<bool>,
}
