use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum LanguageTrackStatus {
    Active,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LanguageTrack {
    pub id: Uuid,
    pub name: String,
    pub status: LanguageTrackStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub weekly_goal_minutes: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateLanguageTrack {
    pub name: String,
    pub weekly_goal_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateLanguageTrack {
    pub name: Option<String>,
    pub weekly_goal_minutes: Option<i32>,
    pub notes: Option<Option<String>>,
}
