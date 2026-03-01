use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use focusvault_core::domain::*;
use focusvault_core::error::{DomainError, DomainResult};
use focusvault_core::repository::Repository;
use focusvault_core::validation::extract_hostname;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn map_db_err(e: sqlx::Error) -> DomainError {
    match &e {
        sqlx::Error::RowNotFound => DomainError::not_found("record", "unknown"),
        sqlx::Error::Database(db_err) => {
            if let Some(constraint) = db_err.constraint() {
                if constraint.contains("single_active") || constraint.starts_with("uq_") {
                    return DomainError::conflict(format!("Constraint violation: {constraint}"));
                }
            }
            DomainError::Database(e.to_string())
        }
        _ => DomainError::Database(e.to_string()),
    }
}

fn row_to_vault_item(row: &sqlx::postgres::PgRow) -> VaultItem {
    let tags: Vec<String> = row.get("tags");
    let item_type_str: String = row.get("item_type");
    let status_str: String = row.get("status");
    let priority_str: String = row.get("priority");
    let source_str: String = row.get("source");

    VaultItem {
        id: row.get("id"),
        item_type: serde_json::from_value(serde_json::Value::String(item_type_str))
            .unwrap_or(VaultItemType::Note),
        url: row.get("url"),
        title: row.get("title"),
        favicon_url: row.get("favicon_url"),
        hostname: row.get("hostname"),
        why: row.get("why"),
        notes: row.get("notes"),
        status: serde_json::from_value(serde_json::Value::String(status_str))
            .unwrap_or(VaultItemStatus::Inbox),
        priority: serde_json::from_value(serde_json::Value::String(priority_str))
            .unwrap_or(Priority::Med),
        pinned: row.get("pinned"),
        tags,
        project_id: row.get("project_id"),
        source: serde_json::from_value(serde_json::Value::String(source_str))
            .unwrap_or(CaptureSource::Manual),
        due_at: row.get("due_at"),
        last_opened_at: row.get("last_opened_at"),
        open_count: row.get("open_count"),
        metadata: row.get("metadata"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_mission(row: &sqlx::postgres::PgRow) -> Mission {
    let status_str: String = row.get("status");
    let weekly_targets: Option<serde_json::Value> = row.get("weekly_targets");
    let kpis: Option<serde_json::Value> = row.get("kpis");

    Mission {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        status: serde_json::from_value(serde_json::Value::String(status_str))
            .unwrap_or(MissionStatus::Paused),
        started_at: row.get("started_at"),
        ended_at: row.get("ended_at"),
        tab_limit: row.get("tab_limit"),
        weekly_targets: weekly_targets.and_then(|v| serde_json::from_value(v).ok()),
        kpis: kpis.and_then(|v| serde_json::from_value(v).ok()),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_daily_log(row: &sqlx::postgres::PgRow) -> DailyLog {
    let mood_str: Option<String> = row.get("mood");

    DailyLog {
        id: row.get("id"),
        date: row.get("date"),
        plan_day: row.get("plan_day"),
        cycles_completed: row.get("cycles_completed"),
        watch_done: row.get("watch_done"),
        build_done: row.get("build_done"),
        prove_done: row.get("prove_done"),
        tab_limit: row.get("tab_limit"),
        tab_limit_met: row.get("tab_limit_met"),
        active_mission_id: row.get("active_mission_id"),
        focus_notes: row.get("focus_notes"),
        sleep_hours_est: row.get("sleep_hours_est"),
        mood: mood_str.and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok()),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_course(row: &sqlx::postgres::PgRow) -> Course {
    let status_str: String = row.get("status");

    Course {
        id: row.get("id"),
        name: row.get("name"),
        provider: row.get("provider"),
        url: row.get("url"),
        status: serde_json::from_value(serde_json::Value::String(status_str))
            .unwrap_or(CourseStatus::Paused),
        started_at: row.get("started_at"),
        completed_at: row.get("completed_at"),
        progress_minutes: row.get("progress_minutes"),
        progress_notes: row.get("progress_notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_language_track(row: &sqlx::postgres::PgRow) -> LanguageTrack {
    let status_str: String = row.get("status");

    LanguageTrack {
        id: row.get("id"),
        name: row.get("name"),
        status: serde_json::from_value(serde_json::Value::String(status_str))
            .unwrap_or(LanguageTrackStatus::Paused),
        started_at: row.get("started_at"),
        completed_at: row.get("completed_at"),
        weekly_goal_minutes: row.get("weekly_goal_minutes"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[async_trait]
impl Repository for PostgresRepo {
    // ── Users ──

    async fn create_user(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
        display_name: Option<&str>,
    ) -> DomainResult<UserRow> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, display_name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $5)",
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(display_name)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("uq_users_email") || e.to_string().contains("duplicate key") {
                DomainError::duplicate("User", email.to_string())
            } else {
                map_db_err(e)
            }
        })?;

        self.get_user_by_id(id)
            .await?
            .ok_or_else(|| DomainError::Internal("Failed to create user".into()))
    }

    async fn get_user_by_email(&self, email: &str) -> DomainResult<Option<UserRow>> {
        let row = sqlx::query("SELECT * FROM users WHERE LOWER(email) = LOWER($1)")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(row.map(|r| UserRow {
            id: r.get("id"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            display_name: r.get("display_name"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn get_user_by_id(&self, id: Uuid) -> DomainResult<Option<UserRow>> {
        let row = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(row.map(|r| UserRow {
            id: r.get("id"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            display_name: r.get("display_name"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    // ── Sync Events ──

    async fn insert_sync_event(&self, event: &SyncEvent) -> DomainResult<()> {
        let entity_type = serde_json::to_value(&event.entity_type).unwrap();
        let action = serde_json::to_value(&event.action).unwrap();

        sqlx::query(
            "INSERT INTO sync_events (id, user_id, entity_type, entity_id, action, payload, device_id, timestamp) \
             VALUES ($1, $2, $3::sync_entity_type, $4, $5::sync_action, $6, $7, $8)",
        )
        .bind(event.id)
        .bind(event.user_id)
        .bind(entity_type.as_str().unwrap())
        .bind(event.entity_id)
        .bind(action.as_str().unwrap())
        .bind(&event.payload)
        .bind(&event.device_id)
        .bind(event.timestamp)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(())
    }

    async fn list_sync_events_since(
        &self,
        user_id: Uuid,
        since: Option<DateTime<Utc>>,
        exclude_device: &str,
        limit: i64,
    ) -> DomainResult<Vec<SyncEvent>> {
        let rows = sqlx::query(
            "SELECT id, user_id, entity_type::text, entity_id, action::text, payload, device_id, timestamp \
             FROM sync_events \
             WHERE user_id = $1 AND device_id != $2 AND ($3::timestamptz IS NULL OR timestamp > $3) \
             ORDER BY timestamp ASC LIMIT $4",
        )
        .bind(user_id)
        .bind(exclude_device)
        .bind(since)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows
            .iter()
            .map(|r| SyncEvent {
                id: r.get("id"),
                user_id: r.get("user_id"),
                entity_type: serde_json::from_value(serde_json::Value::String(
                    r.get::<String, _>("entity_type"),
                ))
                .unwrap(),
                entity_id: r.get("entity_id"),
                action: serde_json::from_value(serde_json::Value::String(
                    r.get::<String, _>("action"),
                ))
                .unwrap(),
                payload: r.get("payload"),
                device_id: r.get("device_id"),
                timestamp: r.get("timestamp"),
            })
            .collect())
    }

    async fn count_pending_sync_events(&self, user_id: Uuid, device_id: &str) -> DomainResult<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as cnt FROM sync_events WHERE user_id = $1 AND device_id != $2",
        )
        .bind(user_id)
        .bind(device_id)
        .fetch_one(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(row.get::<i64, _>("cnt"))
    }

    // ── Vault Items ──

    async fn list_vault_items(&self, filters: VaultFilters) -> DomainResult<Vec<VaultItem>> {
        let limit = filters.limit.unwrap_or(50);
        let offset = filters.offset.unwrap_or(0);

        let select = "SELECT id, item_type::text, url, title, favicon_url, hostname, why, notes, \
             status::text, priority::text, pinned, tags, project_id, source::text, \
             due_at, last_opened_at, open_count, metadata, created_at, updated_at \
             FROM vault_items";

        let mut conditions: Vec<String> = vec![];
        let mut bind_idx = 1u32;

        if filters.status.is_some() {
            conditions.push(format!("status = ${bind_idx}::vault_item_status"));
            bind_idx += 1;
        }
        if filters.priority.is_some() {
            conditions.push(format!("priority = ${bind_idx}::priority"));
            bind_idx += 1;
        }
        if filters.source.is_some() {
            conditions.push(format!("source = ${bind_idx}::capture_source"));
            bind_idx += 1;
        }
        if filters.project_id.is_some() {
            conditions.push(format!("project_id = ${bind_idx}"));
            bind_idx += 1;
        }
        if let Some(ref q) = filters.query {
            if !q.trim().is_empty() {
                conditions.push(format!(
                    "to_tsvector('english', coalesce(title,'') || ' ' || coalesce(why,'') || ' ' || coalesce(notes,'')) @@ plainto_tsquery('english', ${bind_idx})"
                ));
                bind_idx += 1;
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "{select}{where_clause} ORDER BY pinned DESC, created_at DESC LIMIT ${bind_idx} OFFSET ${}",
            bind_idx + 1
        );

        let mut query = sqlx::query(&sql);

        if let Some(ref status) = filters.status {
            let s = serde_json::to_value(status).unwrap();
            query = query.bind(s.as_str().unwrap().to_string());
        }
        if let Some(ref priority) = filters.priority {
            let p = serde_json::to_value(priority).unwrap();
            query = query.bind(p.as_str().unwrap().to_string());
        }
        if let Some(ref source) = filters.source {
            let s = serde_json::to_value(source).unwrap();
            query = query.bind(s.as_str().unwrap().to_string());
        }
        if let Some(ref project_id) = filters.project_id {
            query = query.bind(*project_id);
        }
        if let Some(ref q) = filters.query {
            if !q.trim().is_empty() {
                query = query.bind(q.clone());
            }
        }

        query = query.bind(limit).bind(offset);

        let rows = query.fetch_all(&self.pool).await.map_err(map_db_err)?;
        Ok(rows.iter().map(row_to_vault_item).collect())
    }

    async fn get_vault_item(&self, id: Uuid) -> DomainResult<VaultItem> {
        let row = sqlx::query(
            "SELECT id, item_type::text, url, title, favicon_url, hostname, why, notes, \
             status::text, priority::text, pinned, tags, project_id, source::text, \
             due_at, last_opened_at, open_count, metadata, created_at, updated_at \
             FROM vault_items WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?
        .ok_or_else(|| DomainError::not_found("VaultItem", id.to_string()))?;

        Ok(row_to_vault_item(&row))
    }

    async fn create_vault_item(&self, input: CreateVaultItem) -> DomainResult<VaultItem> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let hostname = input.url.as_deref().and_then(extract_hostname);
        let status = input.status.unwrap_or_default();
        let priority = input.priority.unwrap_or_default();
        let tags = input.tags.unwrap_or_default();
        let source = input.source.unwrap_or_default();

        let item_type_str = serde_json::to_value(&input.item_type)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let status_str = serde_json::to_value(&status)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let priority_str = serde_json::to_value(&priority)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let source_str = serde_json::to_value(&source)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        sqlx::query(
            "INSERT INTO vault_items (id, item_type, url, title, favicon_url, hostname, why, notes, \
             status, priority, pinned, tags, project_id, source, due_at, metadata, created_at, updated_at) \
             VALUES ($1, $2::vault_item_type, $3, $4, $5, $6, $7, $8, \
             $9::vault_item_status, $10::priority, FALSE, $11, $12, $13::capture_source, \
             $14, $15, $16, $16)",
        )
        .bind(id)
        .bind(&item_type_str)
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.favicon_url)
        .bind(&hostname)
        .bind(&input.why)
        .bind(&input.notes)
        .bind(&status_str)
        .bind(&priority_str)
        .bind(&tags)
        .bind(input.project_id)
        .bind(&source_str)
        .bind(input.due_at)
        .bind(&input.metadata)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_vault_item(id).await
    }

    async fn update_vault_item(
        &self,
        id: Uuid,
        update: UpdateVaultItem,
    ) -> DomainResult<VaultItem> {
        let _existing = self.get_vault_item(id).await?;

        if let Some(ref title) = update.title {
            sqlx::query("UPDATE vault_items SET title = $1 WHERE id = $2")
                .bind(title)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref why) = update.why {
            sqlx::query("UPDATE vault_items SET why = $1 WHERE id = $2")
                .bind(why)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref notes) = update.notes {
            sqlx::query("UPDATE vault_items SET notes = $1 WHERE id = $2")
                .bind(notes)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref status) = update.status {
            let s = serde_json::to_value(status)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            sqlx::query("UPDATE vault_items SET status = $1::vault_item_status WHERE id = $2")
                .bind(&s)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref priority) = update.priority {
            let p = serde_json::to_value(priority)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            sqlx::query("UPDATE vault_items SET priority = $1::priority WHERE id = $2")
                .bind(&p)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(pinned) = update.pinned {
            sqlx::query("UPDATE vault_items SET pinned = $1 WHERE id = $2")
                .bind(pinned)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref tags) = update.tags {
            sqlx::query("UPDATE vault_items SET tags = $1 WHERE id = $2")
                .bind(tags)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_vault_item(id).await
    }

    async fn delete_vault_item(&self, id: Uuid) -> DomainResult<()> {
        let result = sqlx::query("DELETE FROM vault_items WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("VaultItem", id.to_string()));
        }
        Ok(())
    }

    async fn stale_inbox_items(&self, hours: i64) -> DomainResult<Vec<VaultItem>> {
        let rows = sqlx::query(
            "SELECT id, item_type::text, url, title, favicon_url, hostname, why, notes, \
             status::text, priority::text, pinned, tags, project_id, source::text, \
             due_at, last_opened_at, open_count, metadata, created_at, updated_at \
             FROM vault_items \
             WHERE status = 'inbox' \
               AND (last_opened_at IS NULL OR last_opened_at < now() - make_interval(hours => $1)) \
               AND created_at < now() - make_interval(hours => $1) \
             ORDER BY created_at ASC",
        )
        .bind(hours as i32)
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_vault_item).collect())
    }

    // ── Projects ──

    async fn list_projects(&self) -> DomainResult<Vec<Project>> {
        let rows = sqlx::query("SELECT * FROM projects ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(rows
            .iter()
            .map(|r| Project {
                id: r.get("id"),
                name: r.get("name"),
                color: r.get("color"),
                archived: r.get("archived"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }

    async fn create_project(&self, input: CreateProject) -> DomainResult<Project> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO projects (id, name, color, created_at, updated_at) VALUES ($1, $2, $3, $4, $4)",
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.color)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(Project {
            id,
            name: input.name,
            color: input.color,
            archived: false,
            created_at: now,
            updated_at: now,
        })
    }

    async fn update_project(&self, id: Uuid, update: UpdateProject) -> DomainResult<Project> {
        if let Some(ref name) = update.name {
            sqlx::query("UPDATE projects SET name = $1 WHERE id = $2")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(archived) = update.archived {
            sqlx::query("UPDATE projects SET archived = $1 WHERE id = $2")
                .bind(archived)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        let row = sqlx::query("SELECT * FROM projects WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| DomainError::not_found("Project", id.to_string()))?;

        Ok(Project {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            archived: row.get("archived"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    // ── Daily Logs ──

    async fn list_daily_logs(&self, filters: DailyLogFilters) -> DomainResult<Vec<DailyLog>> {
        let limit = filters.limit.unwrap_or(30);
        let offset = filters.offset.unwrap_or(0);

        let rows = sqlx::query(
            "SELECT id, date, plan_day, cycles_completed, watch_done, build_done, prove_done, \
             tab_limit, tab_limit_met, active_mission_id, focus_notes, sleep_hours_est, \
             mood::text, created_at, updated_at \
             FROM daily_logs \
             WHERE ($1::date IS NULL OR date >= $1) AND ($2::date IS NULL OR date <= $2) \
             ORDER BY date DESC LIMIT $3 OFFSET $4",
        )
        .bind(filters.from)
        .bind(filters.to)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_daily_log).collect())
    }

    async fn get_daily_log_by_date(&self, date: NaiveDate) -> DomainResult<Option<DailyLog>> {
        let row = sqlx::query(
            "SELECT id, date, plan_day, cycles_completed, watch_done, build_done, prove_done, \
             tab_limit, tab_limit_met, active_mission_id, focus_notes, sleep_hours_est, \
             mood::text, created_at, updated_at \
             FROM daily_logs WHERE date = $1",
        )
        .bind(date)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(row.as_ref().map(row_to_daily_log))
    }

    async fn upsert_daily_log(&self, input: UpsertDailyLog) -> DomainResult<DailyLog> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let mood_str = input
            .mood
            .as_ref()
            .map(|m| serde_json::to_value(m).unwrap().as_str().unwrap().to_string());

        sqlx::query(
            "INSERT INTO daily_logs (id, date, plan_day, cycles_completed, watch_done, build_done, prove_done, \
             tab_limit, tab_limit_met, active_mission_id, focus_notes, sleep_hours_est, mood, created_at, updated_at) \
             VALUES ($1, $2, $3, COALESCE($4, 0), COALESCE($5, FALSE), COALESCE($6, FALSE), COALESCE($7, FALSE), \
             COALESCE($8, 10), $9, $10, $11, $12, $13::mood, $14, $14) \
             ON CONFLICT (date) DO UPDATE SET \
             plan_day = COALESCE(EXCLUDED.plan_day, daily_logs.plan_day), \
             cycles_completed = COALESCE(EXCLUDED.cycles_completed, daily_logs.cycles_completed), \
             watch_done = COALESCE(EXCLUDED.watch_done, daily_logs.watch_done), \
             build_done = COALESCE(EXCLUDED.build_done, daily_logs.build_done), \
             prove_done = COALESCE(EXCLUDED.prove_done, daily_logs.prove_done)"
        )
        .bind(id)
        .bind(input.date)
        .bind(input.plan_day)
        .bind(input.cycles_completed)
        .bind(input.watch_done)
        .bind(input.build_done)
        .bind(input.prove_done)
        .bind(input.tab_limit)
        .bind(input.tab_limit_met)
        .bind(input.active_mission_id)
        .bind(&input.focus_notes)
        .bind(input.sleep_hours_est)
        .bind(&mood_str)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_daily_log_by_date(input.date)
            .await?
            .ok_or_else(|| DomainError::Internal("Failed to upsert daily log".into()))
    }

    async fn update_daily_log(
        &self,
        date: NaiveDate,
        update: UpdateDailyLog,
    ) -> DomainResult<DailyLog> {
        let _existing = self
            .get_daily_log_by_date(date)
            .await?
            .ok_or_else(|| DomainError::not_found("DailyLog", date.to_string()))?;

        if let Some(cycles) = update.cycles_completed {
            sqlx::query("UPDATE daily_logs SET cycles_completed = $1 WHERE date = $2")
                .bind(cycles)
                .bind(date)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(watch) = update.watch_done {
            sqlx::query("UPDATE daily_logs SET watch_done = $1 WHERE date = $2")
                .bind(watch)
                .bind(date)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(build) = update.build_done {
            sqlx::query("UPDATE daily_logs SET build_done = $1 WHERE date = $2")
                .bind(build)
                .bind(date)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(prove) = update.prove_done {
            sqlx::query("UPDATE daily_logs SET prove_done = $1 WHERE date = $2")
                .bind(prove)
                .bind(date)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_daily_log_by_date(date)
            .await?
            .ok_or_else(|| DomainError::not_found("DailyLog", date.to_string()))
    }

    // ── Missions ──

    async fn list_missions(&self) -> DomainResult<Vec<Mission>> {
        let rows = sqlx::query(
            "SELECT id, name, description, status::text, started_at, ended_at, tab_limit, \
             weekly_targets, kpis, created_at, updated_at \
             FROM missions ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_mission).collect())
    }

    async fn get_mission(&self, id: Uuid) -> DomainResult<Mission> {
        let row = sqlx::query(
            "SELECT id, name, description, status::text, started_at, ended_at, tab_limit, \
             weekly_targets, kpis, created_at, updated_at \
             FROM missions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?
        .ok_or_else(|| DomainError::not_found("Mission", id.to_string()))?;

        Ok(row_to_mission(&row))
    }

    async fn create_mission(&self, input: CreateMission) -> DomainResult<Mission> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let tab_limit = input.tab_limit.unwrap_or(10);
        let weekly_targets = input
            .weekly_targets
            .map(|wt| serde_json::to_value(wt).unwrap());
        let kpis = input.kpis.map(|k| serde_json::to_value(k).unwrap());

        sqlx::query(
            "INSERT INTO missions (id, name, description, status, tab_limit, weekly_targets, kpis, created_at, updated_at) \
             VALUES ($1, $2, $3, 'paused', $4, $5, $6, $7, $7)",
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(tab_limit)
        .bind(&weekly_targets)
        .bind(&kpis)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_mission(id).await
    }

    async fn update_mission(&self, id: Uuid, update: UpdateMission) -> DomainResult<Mission> {
        let _existing = self.get_mission(id).await?;

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE missions SET name = $1 WHERE id = $2")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref status) = update.status {
            let s = serde_json::to_value(status)
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            sqlx::query("UPDATE missions SET status = $1::mission_status WHERE id = $2")
                .bind(&s)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_mission(id).await
    }

    async fn get_active_mission(&self) -> DomainResult<Option<Mission>> {
        let row = sqlx::query(
            "SELECT id, name, description, status::text, started_at, ended_at, tab_limit, \
             weekly_targets, kpis, created_at, updated_at \
             FROM missions WHERE status = 'active' LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(row.as_ref().map(row_to_mission))
    }

    async fn deactivate_all_missions(&self) -> DomainResult<()> {
        sqlx::query("UPDATE missions SET status = 'paused' WHERE status = 'active'")
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;
        Ok(())
    }

    async fn activate_mission(&self, id: Uuid) -> DomainResult<Mission> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE missions SET status = 'active', started_at = COALESCE(started_at, $1) WHERE id = $2",
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_mission(id).await
    }

    // ── Courses ──

    async fn list_courses(&self) -> DomainResult<Vec<Course>> {
        let rows = sqlx::query(
            "SELECT id, name, provider, url, status::text, started_at, completed_at, \
             progress_minutes, progress_notes, created_at, updated_at \
             FROM courses ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_course).collect())
    }

    async fn get_course(&self, id: Uuid) -> DomainResult<Course> {
        let row = sqlx::query(
            "SELECT id, name, provider, url, status::text, started_at, completed_at, \
             progress_minutes, progress_notes, created_at, updated_at \
             FROM courses WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?
        .ok_or_else(|| DomainError::not_found("Course", id.to_string()))?;

        Ok(row_to_course(&row))
    }

    async fn create_course(&self, input: CreateCourse) -> DomainResult<Course> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO courses (id, name, provider, url, status, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, 'paused', $5, $5)",
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.provider)
        .bind(&input.url)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_course(id).await
    }

    async fn update_course(&self, id: Uuid, update: UpdateCourse) -> DomainResult<Course> {
        let _existing = self.get_course(id).await?;

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE courses SET name = $1 WHERE id = $2")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(minutes) = update.progress_minutes {
            sqlx::query("UPDATE courses SET progress_minutes = $1 WHERE id = $2")
                .bind(minutes)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_course(id).await
    }

    async fn get_active_course(&self) -> DomainResult<Option<Course>> {
        let row = sqlx::query(
            "SELECT id, name, provider, url, status::text, started_at, completed_at, \
             progress_minutes, progress_notes, created_at, updated_at \
             FROM courses WHERE status = 'active' LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(row.as_ref().map(row_to_course))
    }

    async fn deactivate_all_courses(&self) -> DomainResult<()> {
        sqlx::query("UPDATE courses SET status = 'paused' WHERE status = 'active'")
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;
        Ok(())
    }

    async fn activate_course(&self, id: Uuid) -> DomainResult<Course> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE courses SET status = 'active', started_at = COALESCE(started_at, $1) WHERE id = $2",
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_course(id).await
    }

    async fn complete_course(&self, id: Uuid) -> DomainResult<Course> {
        let now = Utc::now();
        sqlx::query("UPDATE courses SET status = 'completed', completed_at = $1 WHERE id = $2")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_course(id).await
    }

    // ── Language Tracks ──

    async fn list_language_tracks(&self) -> DomainResult<Vec<LanguageTrack>> {
        let rows = sqlx::query(
            "SELECT id, name, status::text, started_at, completed_at, weekly_goal_minutes, \
             notes, created_at, updated_at \
             FROM language_tracks ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_language_track).collect())
    }

    async fn get_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        let row = sqlx::query(
            "SELECT id, name, status::text, started_at, completed_at, weekly_goal_minutes, \
             notes, created_at, updated_at \
             FROM language_tracks WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?
        .ok_or_else(|| DomainError::not_found("LanguageTrack", id.to_string()))?;

        Ok(row_to_language_track(&row))
    }

    async fn create_language_track(
        &self,
        input: CreateLanguageTrack,
    ) -> DomainResult<LanguageTrack> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let weekly_goal = input.weekly_goal_minutes.unwrap_or(420);

        sqlx::query(
            "INSERT INTO language_tracks (id, name, status, weekly_goal_minutes, created_at, updated_at) \
             VALUES ($1, $2, 'paused', $3, $4, $4)",
        )
        .bind(id)
        .bind(&input.name)
        .bind(weekly_goal)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_language_track(id).await
    }

    async fn update_language_track(
        &self,
        id: Uuid,
        update: UpdateLanguageTrack,
    ) -> DomainResult<LanguageTrack> {
        let _existing = self.get_language_track(id).await?;

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE language_tracks SET name = $1 WHERE id = $2")
                .bind(name)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(minutes) = update.weekly_goal_minutes {
            sqlx::query("UPDATE language_tracks SET weekly_goal_minutes = $1 WHERE id = $2")
                .bind(minutes)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_language_track(id).await
    }

    async fn get_active_language_track(&self) -> DomainResult<Option<LanguageTrack>> {
        let row = sqlx::query(
            "SELECT id, name, status::text, started_at, completed_at, weekly_goal_minutes, \
             notes, created_at, updated_at \
             FROM language_tracks WHERE status = 'active' LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(row.as_ref().map(row_to_language_track))
    }

    async fn deactivate_all_language_tracks(&self) -> DomainResult<()> {
        sqlx::query("UPDATE language_tracks SET status = 'paused' WHERE status = 'active'")
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;
        Ok(())
    }

    async fn activate_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE language_tracks SET status = 'active', started_at = COALESCE(started_at, $1) WHERE id = $2",
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_language_track(id).await
    }

    async fn complete_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE language_tracks SET status = 'completed', completed_at = $1 WHERE id = $2",
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        self.get_language_track(id).await
    }
}
