use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MissionStatus {
    Active,
    Paused,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WeeklyTargets {
    #[serde(default)]
    pub deploy: bool,
    #[serde(default)]
    pub outreach: bool,
    #[serde(default)]
    pub iterate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MissionKpis {
    pub calls_booked: Option<f64>,
    pub revenue: Option<f64>,
    pub leads: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: MissionStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub tab_limit: i32,
    pub weekly_targets: Option<WeeklyTargets>,
    pub kpis: Option<MissionKpis>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateMission {
    pub name: String,
    pub description: Option<String>,
    pub tab_limit: Option<i32>,
    pub weekly_targets: Option<WeeklyTargets>,
    pub kpis: Option<MissionKpis>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateMission {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<MissionStatus>,
    pub tab_limit: Option<i32>,
    pub weekly_targets: Option<WeeklyTargets>,
    pub kpis: Option<Option<MissionKpis>>,
}
