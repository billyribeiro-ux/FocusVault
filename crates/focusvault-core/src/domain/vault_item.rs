use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VaultItemType {
    Link,
    Note,
    Snippet,
    FileRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VaultItemStatus {
    Inbox,
    Saved,
    Actioned,
    Archived,
}

impl Default for VaultItemStatus {
    fn default() -> Self {
        Self::Inbox
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Med,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Med
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSource {
    Manual,
    Extension,
    Import,
    Api,
}

impl Default for CaptureSource {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VaultItem {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub item_type: VaultItemType,
    pub url: Option<String>,
    pub title: Option<String>,
    pub favicon_url: Option<String>,
    pub hostname: Option<String>,
    pub why: String,
    pub notes: Option<String>,
    pub status: VaultItemStatus,
    pub priority: Priority,
    pub pinned: bool,
    pub tags: Vec<String>,
    pub project_id: Option<Uuid>,
    pub source: CaptureSource,
    pub due_at: Option<DateTime<Utc>>,
    pub last_opened_at: Option<DateTime<Utc>>,
    pub open_count: i32,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new vault item.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateVaultItem {
    #[serde(rename = "type")]
    pub item_type: VaultItemType,
    pub url: Option<String>,
    pub title: Option<String>,
    pub favicon_url: Option<String>,
    pub why: String,
    pub notes: Option<String>,
    pub status: Option<VaultItemStatus>,
    pub priority: Option<Priority>,
    pub tags: Option<Vec<String>>,
    pub project_id: Option<Uuid>,
    pub source: Option<CaptureSource>,
    pub due_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

/// Input for updating a vault item (all fields optional).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateVaultItem {
    pub title: Option<String>,
    pub why: Option<String>,
    pub notes: Option<String>,
    pub status: Option<VaultItemStatus>,
    pub priority: Option<Priority>,
    pub pinned: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub project_id: Option<Option<Uuid>>,
    pub due_at: Option<Option<DateTime<Utc>>>,
    pub metadata: Option<serde_json::Value>,
}

/// Filters for listing vault items.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct VaultFilters {
    pub query: Option<String>,
    pub status: Option<VaultItemStatus>,
    pub project_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<Priority>,
    pub source: Option<CaptureSource>,
    pub has_due_date: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
