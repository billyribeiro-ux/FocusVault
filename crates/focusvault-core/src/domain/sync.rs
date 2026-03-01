use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Represents a tracked change for offline-first sync.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub entity_type: SyncEntityType,
    pub entity_id: Uuid,
    pub action: SyncAction,
    pub payload: serde_json::Value,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SyncEntityType {
    VaultItem,
    Mission,
    DailyLog,
    Course,
    LanguageTrack,
    Project,
    Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SyncAction {
    Create,
    Update,
    Delete,
}

/// Request to push local changes to the server.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncPushRequest {
    pub device_id: String,
    pub events: Vec<SyncPushEvent>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

/// A single change event to push.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncPushEvent {
    pub entity_type: SyncEntityType,
    pub entity_id: Uuid,
    pub action: SyncAction,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

/// Response after pushing changes.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncPushResponse {
    pub accepted: i32,
    pub conflicts: Vec<SyncConflict>,
    pub server_time: DateTime<Utc>,
}

/// A conflict detected during sync push.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncConflict {
    pub entity_type: SyncEntityType,
    pub entity_id: Uuid,
    pub client_version: serde_json::Value,
    pub server_version: serde_json::Value,
}

/// Request to pull changes from the server since a given timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncPullRequest {
    pub device_id: String,
    pub last_synced_at: Option<DateTime<Utc>>,
}

/// Response with changes since the given timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncPullResponse {
    pub events: Vec<SyncEvent>,
    pub server_time: DateTime<Utc>,
    pub has_more: bool,
}

/// Sync status for the current device.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncStatus {
    pub device_id: String,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub pending_changes: i32,
    pub is_online: bool,
}
