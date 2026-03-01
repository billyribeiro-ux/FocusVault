use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CourseStatus {
    Active,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub provider: Option<String>,
    pub url: Option<String>,
    pub status: CourseStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress_minutes: i32,
    pub progress_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCourse {
    pub name: String,
    pub provider: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub provider: Option<Option<String>>,
    pub url: Option<Option<String>>,
    pub progress_minutes: Option<i32>,
    pub progress_notes: Option<Option<String>>,
}
