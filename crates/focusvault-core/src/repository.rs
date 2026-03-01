use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::*;
use crate::error::DomainResult;

/// Database-agnostic repository trait.
/// Implemented by PostgresRepo and SqliteRepo.
#[async_trait]
pub trait Repository: Send + Sync + 'static {
    // ── Users ──

    async fn create_user(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
        display_name: Option<&str>,
    ) -> DomainResult<UserRow>;
    async fn get_user_by_email(&self, email: &str) -> DomainResult<Option<UserRow>>;
    async fn get_user_by_id(&self, id: Uuid) -> DomainResult<Option<UserRow>>;

    // ── Sync Events ──

    async fn insert_sync_event(&self, event: &SyncEvent) -> DomainResult<()>;
    async fn list_sync_events_since(
        &self,
        user_id: Uuid,
        since: Option<DateTime<Utc>>,
        exclude_device: &str,
        limit: i64,
    ) -> DomainResult<Vec<SyncEvent>>;
    async fn count_pending_sync_events(&self, user_id: Uuid, device_id: &str) -> DomainResult<i64>;

    // ── Vault Items ──

    async fn list_vault_items(&self, filters: VaultFilters) -> DomainResult<Vec<VaultItem>>;
    async fn get_vault_item(&self, id: Uuid) -> DomainResult<VaultItem>;
    async fn create_vault_item(&self, item: CreateVaultItem) -> DomainResult<VaultItem>;
    async fn update_vault_item(&self, id: Uuid, update: UpdateVaultItem)
        -> DomainResult<VaultItem>;
    async fn delete_vault_item(&self, id: Uuid) -> DomainResult<()>;

    /// Count items in inbox that haven't been opened in `hours` hours.
    async fn stale_inbox_items(&self, hours: i64) -> DomainResult<Vec<VaultItem>>;

    // ── Projects ──

    async fn list_projects(&self) -> DomainResult<Vec<Project>>;
    async fn create_project(&self, input: CreateProject) -> DomainResult<Project>;
    async fn update_project(&self, id: Uuid, update: UpdateProject) -> DomainResult<Project>;

    // ── Daily Logs ──

    async fn list_daily_logs(&self, filters: DailyLogFilters) -> DomainResult<Vec<DailyLog>>;
    async fn get_daily_log_by_date(&self, date: NaiveDate) -> DomainResult<Option<DailyLog>>;
    async fn upsert_daily_log(&self, input: UpsertDailyLog) -> DomainResult<DailyLog>;
    async fn update_daily_log(
        &self,
        date: NaiveDate,
        update: UpdateDailyLog,
    ) -> DomainResult<DailyLog>;

    // ── Missions ──

    async fn list_missions(&self) -> DomainResult<Vec<Mission>>;
    async fn get_mission(&self, id: Uuid) -> DomainResult<Mission>;
    async fn create_mission(&self, input: CreateMission) -> DomainResult<Mission>;
    async fn update_mission(&self, id: Uuid, update: UpdateMission) -> DomainResult<Mission>;
    async fn get_active_mission(&self) -> DomainResult<Option<Mission>>;
    async fn deactivate_all_missions(&self) -> DomainResult<()>;
    async fn activate_mission(&self, id: Uuid) -> DomainResult<Mission>;

    // ── Courses ──

    async fn list_courses(&self) -> DomainResult<Vec<Course>>;
    async fn get_course(&self, id: Uuid) -> DomainResult<Course>;
    async fn create_course(&self, input: CreateCourse) -> DomainResult<Course>;
    async fn update_course(&self, id: Uuid, update: UpdateCourse) -> DomainResult<Course>;
    async fn get_active_course(&self) -> DomainResult<Option<Course>>;
    async fn deactivate_all_courses(&self) -> DomainResult<()>;
    async fn activate_course(&self, id: Uuid) -> DomainResult<Course>;
    async fn complete_course(&self, id: Uuid) -> DomainResult<Course>;

    // ── Language Tracks ──

    async fn list_language_tracks(&self) -> DomainResult<Vec<LanguageTrack>>;
    async fn get_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack>;
    async fn create_language_track(
        &self,
        input: CreateLanguageTrack,
    ) -> DomainResult<LanguageTrack>;
    async fn update_language_track(
        &self,
        id: Uuid,
        update: UpdateLanguageTrack,
    ) -> DomainResult<LanguageTrack>;
    async fn get_active_language_track(&self) -> DomainResult<Option<LanguageTrack>>;
    async fn deactivate_all_language_tracks(&self) -> DomainResult<()>;
    async fn activate_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack>;
    async fn complete_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack>;
}
