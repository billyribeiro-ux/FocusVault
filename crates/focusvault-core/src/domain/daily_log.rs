use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    Low,
    Ok,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DailyLog {
    pub id: Uuid,
    pub date: NaiveDate,
    pub plan_day: Option<i32>,
    pub cycles_completed: i32,
    pub watch_done: bool,
    pub build_done: bool,
    pub prove_done: bool,
    pub tab_limit: i32,
    pub tab_limit_met: Option<bool>,
    pub active_mission_id: Option<Uuid>,
    pub focus_notes: Option<String>,
    pub sleep_hours_est: Option<f64>,
    pub mood: Option<Mood>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for upserting a daily log (creates if not exists for the date).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpsertDailyLog {
    pub date: NaiveDate,
    pub plan_day: Option<i32>,
    pub cycles_completed: Option<i32>,
    pub watch_done: Option<bool>,
    pub build_done: Option<bool>,
    pub prove_done: Option<bool>,
    pub tab_limit: Option<i32>,
    pub tab_limit_met: Option<bool>,
    pub active_mission_id: Option<Uuid>,
    pub focus_notes: Option<String>,
    pub sleep_hours_est: Option<f64>,
    pub mood: Option<Mood>,
}

/// Input for updating an existing daily log.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateDailyLog {
    pub plan_day: Option<Option<i32>>,
    pub cycles_completed: Option<i32>,
    pub watch_done: Option<bool>,
    pub build_done: Option<bool>,
    pub prove_done: Option<bool>,
    pub tab_limit: Option<i32>,
    pub tab_limit_met: Option<Option<bool>>,
    pub focus_notes: Option<Option<String>>,
    pub sleep_hours_est: Option<Option<f64>>,
    pub mood: Option<Option<Mood>>,
}

/// Filters for listing daily logs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct DailyLogFilters {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
