use std::sync::Arc;

use chrono::NaiveDate;

use crate::domain::*;
use crate::error::DomainResult;
use crate::repository::Repository;

pub struct DailyLogService {
    repo: Arc<dyn Repository>,
}

impl DailyLogService {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self, filters: DailyLogFilters) -> DomainResult<Vec<DailyLog>> {
        self.repo.list_daily_logs(filters).await
    }

    /// Get or create the daily log for a given date.
    /// Implements the "auto-create on first open" requirement.
    pub async fn get_or_create(&self, date: NaiveDate) -> DomainResult<DailyLog> {
        if let Some(log) = self.repo.get_daily_log_by_date(date).await? {
            return Ok(log);
        }

        // Auto-create with defaults
        let input = UpsertDailyLog {
            date,
            plan_day: None,
            cycles_completed: None,
            watch_done: None,
            build_done: None,
            prove_done: None,
            tab_limit: None,
            tab_limit_met: None,
            active_mission_id: None,
            focus_notes: None,
            sleep_hours_est: None,
            mood: None,
        };

        self.repo.upsert_daily_log(input).await
    }

    pub async fn upsert(&self, input: UpsertDailyLog) -> DomainResult<DailyLog> {
        // Validate plan_day range
        if let Some(day) = input.plan_day {
            if !(1..=30).contains(&day) {
                return Err(crate::error::DomainError::validation(
                    "Plan day must be between 1 and 30",
                ));
            }
        }

        self.repo.upsert_daily_log(input).await
    }

    pub async fn update(&self, date: NaiveDate, update: UpdateDailyLog) -> DomainResult<DailyLog> {
        self.repo.update_daily_log(date, update).await
    }
}
