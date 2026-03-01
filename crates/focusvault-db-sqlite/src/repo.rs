use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use focusvault_core::domain::*;
use focusvault_core::error::{DomainError, DomainResult};
use focusvault_core::repository::Repository;
use focusvault_core::validation::extract_hostname;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct SqliteRepo {
    pool: SqlitePool,
}

impl SqliteRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn map_db_err(e: sqlx::Error) -> DomainError {
    match &e {
        sqlx::Error::RowNotFound => DomainError::not_found("record", "unknown"),
        _ => DomainError::Database(e.to_string()),
    }
}

// Helper to parse a SQLite TEXT column into a DateTime<Utc>
fn parse_dt(s: &str) -> chrono::DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
                .map(|ndt| ndt.and_utc())
                .unwrap_or_else(|_| Utc::now())
        })
}

fn parse_opt_dt(s: Option<&str>) -> Option<chrono::DateTime<Utc>> {
    s.map(parse_dt)
}

fn row_to_vault_item(row: &sqlx::sqlite::SqliteRow) -> VaultItem {
    let tags_json: String = row.get("tags");
    let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
    let metadata_json: Option<String> = row.get("metadata");
    let metadata: Option<serde_json::Value> =
        metadata_json.and_then(|s| serde_json::from_str(&s).ok());
    let project_id_str: Option<String> = row.get("project_id");

    VaultItem {
        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
        item_type: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("item_type")))
            .unwrap_or(VaultItemType::Note),
        url: row.get("url"),
        title: row.get("title"),
        favicon_url: row.get("favicon_url"),
        hostname: row.get("hostname"),
        why: row.get("why"),
        notes: row.get("notes"),
        status: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("status")))
            .unwrap_or(VaultItemStatus::Inbox),
        priority: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("priority")))
            .unwrap_or(Priority::Med),
        pinned: row.get::<i32, _>("pinned") != 0,
        tags,
        project_id: project_id_str.and_then(|s| Uuid::parse_str(&s).ok()),
        source: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("source")))
            .unwrap_or(CaptureSource::Manual),
        due_at: parse_opt_dt(row.get::<Option<&str>, _>("due_at")),
        last_opened_at: parse_opt_dt(row.get::<Option<&str>, _>("last_opened_at")),
        open_count: row.get("open_count"),
        metadata,
        created_at: parse_dt(row.get("created_at")),
        updated_at: parse_dt(row.get("updated_at")),
    }
}

fn row_to_mission(row: &sqlx::sqlite::SqliteRow) -> Mission {
    let weekly_targets_json: Option<String> = row.get("weekly_targets");
    let kpis_json: Option<String> = row.get("kpis");

    Mission {
        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
        name: row.get("name"),
        description: row.get("description"),
        status: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("status")))
            .unwrap_or(MissionStatus::Paused),
        started_at: parse_opt_dt(row.get::<Option<&str>, _>("started_at")),
        ended_at: parse_opt_dt(row.get::<Option<&str>, _>("ended_at")),
        tab_limit: row.get("tab_limit"),
        weekly_targets: weekly_targets_json.and_then(|s| serde_json::from_str(&s).ok()),
        kpis: kpis_json.and_then(|s| serde_json::from_str(&s).ok()),
        created_at: parse_dt(row.get("created_at")),
        updated_at: parse_dt(row.get("updated_at")),
    }
}

fn row_to_daily_log(row: &sqlx::sqlite::SqliteRow) -> DailyLog {
    DailyLog {
        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
        date: NaiveDate::parse_from_str(row.get::<&str, _>("date"), "%Y-%m-%d")
            .unwrap_or_else(|_| Utc::now().date_naive()),
        plan_day: row.get("plan_day"),
        cycles_completed: row.get("cycles_completed"),
        watch_done: row.get::<i32, _>("watch_done") != 0,
        build_done: row.get::<i32, _>("build_done") != 0,
        prove_done: row.get::<i32, _>("prove_done") != 0,
        tab_limit: row.get("tab_limit"),
        tab_limit_met: row
            .get::<Option<i32>, _>("tab_limit_met")
            .map(|v| v != 0),
        active_mission_id: row
            .get::<Option<String>, _>("active_mission_id")
            .and_then(|s| Uuid::parse_str(&s).ok()),
        focus_notes: row.get("focus_notes"),
        sleep_hours_est: row.get("sleep_hours_est"),
        mood: row
            .get::<Option<String>, _>("mood")
            .and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok()),
        created_at: parse_dt(row.get("created_at")),
        updated_at: parse_dt(row.get("updated_at")),
    }
}

fn row_to_course(row: &sqlx::sqlite::SqliteRow) -> Course {
    Course {
        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
        name: row.get("name"),
        provider: row.get("provider"),
        url: row.get("url"),
        status: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("status")))
            .unwrap_or(CourseStatus::Paused),
        started_at: parse_opt_dt(row.get::<Option<&str>, _>("started_at")),
        completed_at: parse_opt_dt(row.get::<Option<&str>, _>("completed_at")),
        progress_minutes: row.get("progress_minutes"),
        progress_notes: row.get("progress_notes"),
        created_at: parse_dt(row.get("created_at")),
        updated_at: parse_dt(row.get("updated_at")),
    }
}

fn row_to_language_track(row: &sqlx::sqlite::SqliteRow) -> LanguageTrack {
    LanguageTrack {
        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
        name: row.get("name"),
        status: serde_json::from_value(serde_json::Value::String(row.get::<String, _>("status")))
            .unwrap_or(LanguageTrackStatus::Paused),
        started_at: parse_opt_dt(row.get::<Option<&str>, _>("started_at")),
        completed_at: parse_opt_dt(row.get::<Option<&str>, _>("completed_at")),
        weekly_goal_minutes: row.get("weekly_goal_minutes"),
        notes: row.get("notes"),
        created_at: parse_dt(row.get("created_at")),
        updated_at: parse_dt(row.get("updated_at")),
    }
}

#[async_trait]
impl Repository for SqliteRepo {
    // ── Users ──

    async fn create_user(
        &self,
        id: Uuid,
        email: &str,
        password_hash: &str,
        display_name: Option<&str>,
    ) -> DomainResult<UserRow> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, display_name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        )
        .bind(id.to_string())
        .bind(email)
        .bind(password_hash)
        .bind(display_name)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
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
        let row = sqlx::query("SELECT * FROM users WHERE email = ?1 COLLATE NOCASE")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(row.map(|r| UserRow {
            id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            display_name: r.get("display_name"),
            created_at: parse_dt(r.get("created_at")),
            updated_at: parse_dt(r.get("updated_at")),
        }))
    }

    async fn get_user_by_id(&self, id: Uuid) -> DomainResult<Option<UserRow>> {
        let row = sqlx::query("SELECT * FROM users WHERE id = ?1")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(row.map(|r| UserRow {
            id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            display_name: r.get("display_name"),
            created_at: parse_dt(r.get("created_at")),
            updated_at: parse_dt(r.get("updated_at")),
        }))
    }

    // ── Sync Events ──

    async fn insert_sync_event(&self, event: &SyncEvent) -> DomainResult<()> {
        let entity_type = serde_json::to_value(&event.entity_type).unwrap();
        let action = serde_json::to_value(&event.action).unwrap();
        let payload = serde_json::to_string(&event.payload).unwrap();

        sqlx::query(
            "INSERT INTO sync_events (id, user_id, entity_type, entity_id, action, payload, device_id, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .bind(event.id.to_string())
        .bind(event.user_id.to_string())
        .bind(entity_type.as_str().unwrap())
        .bind(event.entity_id.to_string())
        .bind(action.as_str().unwrap())
        .bind(&payload)
        .bind(&event.device_id)
        .bind(event.timestamp.to_rfc3339())
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
        let rows = if let Some(since) = since {
            sqlx::query(
                "SELECT * FROM sync_events WHERE user_id = ?1 AND device_id != ?2 AND timestamp > ?3 ORDER BY timestamp ASC LIMIT ?4",
            )
            .bind(user_id.to_string())
            .bind(exclude_device)
            .bind(since.to_rfc3339())
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(map_db_err)?
        } else {
            sqlx::query(
                "SELECT * FROM sync_events WHERE user_id = ?1 AND device_id != ?2 ORDER BY timestamp ASC LIMIT ?3",
            )
            .bind(user_id.to_string())
            .bind(exclude_device)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(map_db_err)?
        };

        Ok(rows
            .iter()
            .map(|r| SyncEvent {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                user_id: Uuid::parse_str(r.get::<&str, _>("user_id")).unwrap(),
                entity_type: serde_json::from_value(serde_json::Value::String(
                    r.get::<String, _>("entity_type"),
                ))
                .unwrap(),
                entity_id: Uuid::parse_str(r.get::<&str, _>("entity_id")).unwrap(),
                action: serde_json::from_value(serde_json::Value::String(
                    r.get::<String, _>("action"),
                ))
                .unwrap(),
                payload: serde_json::from_str(r.get::<&str, _>("payload")).unwrap_or_default(),
                device_id: r.get("device_id"),
                timestamp: parse_dt(r.get("timestamp")),
            })
            .collect())
    }

    async fn count_pending_sync_events(&self, user_id: Uuid, device_id: &str) -> DomainResult<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as cnt FROM sync_events WHERE user_id = ?1 AND device_id != ?2",
        )
        .bind(user_id.to_string())
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

        // Build dynamic WHERE clauses
        let mut conditions: Vec<String> = vec![];
        let mut bind_idx = 1;

        if filters.status.is_some() {
            conditions.push(format!("status = ?{bind_idx}"));
            bind_idx += 1;
        }
        if filters.priority.is_some() {
            conditions.push(format!("priority = ?{bind_idx}"));
            bind_idx += 1;
        }
        if filters.source.is_some() {
            conditions.push(format!("source = ?{bind_idx}"));
            bind_idx += 1;
        }
        if filters.project_id.is_some() {
            conditions.push(format!("project_id = ?{bind_idx}"));
            bind_idx += 1;
        }
        if let Some(ref query) = filters.query {
            if !query.trim().is_empty() {
                conditions.push(format!(
                    "(title LIKE ?{bind_idx} OR why LIKE ?{bind_idx} OR notes LIKE ?{bind_idx} OR url LIKE ?{bind_idx})"
                ));
                bind_idx += 1;
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT * FROM vault_items {where_clause} ORDER BY pinned DESC, created_at DESC LIMIT ?{bind_idx} OFFSET ?{}",
            bind_idx + 1
        );

        let mut query = sqlx::query(&sql);

        // Bind parameters in same order as conditions
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
            query = query.bind(project_id.to_string());
        }
        if let Some(ref q) = filters.query {
            if !q.trim().is_empty() {
                query = query.bind(format!("%{q}%"));
            }
        }

        query = query.bind(limit).bind(offset);

        let rows = query.fetch_all(&self.pool).await.map_err(map_db_err)?;
        Ok(rows.iter().map(row_to_vault_item).collect())
    }

    async fn get_vault_item(&self, id: Uuid) -> DomainResult<VaultItem> {
        let row = sqlx::query("SELECT * FROM vault_items WHERE id = ?1")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| DomainError::not_found("VaultItem", id.to_string()))?;

        Ok(row_to_vault_item(&row))
    }

    async fn create_vault_item(&self, input: CreateVaultItem) -> DomainResult<VaultItem> {
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        let hostname = input.url.as_deref().and_then(extract_hostname);
        let status = input.status.unwrap_or_default();
        let priority = input.priority.unwrap_or_default();
        let tags = serde_json::to_string(&input.tags.unwrap_or_default()).unwrap();
        let source = input.source.unwrap_or_default();
        let metadata = input.metadata.map(|m| serde_json::to_string(&m).unwrap());

        let item_type_str = serde_json::to_value(&input.item_type).unwrap();
        let status_str = serde_json::to_value(&status).unwrap();
        let priority_str = serde_json::to_value(&priority).unwrap();
        let source_str = serde_json::to_value(&source).unwrap();

        sqlx::query(
            "INSERT INTO vault_items (id, item_type, url, title, favicon_url, hostname, why, notes, status, priority, pinned, tags, project_id, source, due_at, metadata, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 0, ?11, ?12, ?13, ?14, ?15, ?16, ?16)"
        )
        .bind(id.to_string())
        .bind(item_type_str.as_str().unwrap())
        .bind(&input.url)
        .bind(&input.title)
        .bind(&input.favicon_url)
        .bind(&hostname)
        .bind(&input.why)
        .bind(&input.notes)
        .bind(status_str.as_str().unwrap())
        .bind(priority_str.as_str().unwrap())
        .bind(&tags)
        .bind(input.project_id.map(|p| p.to_string()))
        .bind(source_str.as_str().unwrap())
        .bind(input.due_at.map(|d| d.to_rfc3339()))
        .bind(&metadata)
        .bind(&now)
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
        let id_str = id.to_string();

        if let Some(ref title) = update.title {
            sqlx::query("UPDATE vault_items SET title = ?1 WHERE id = ?2")
                .bind(title)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref why) = update.why {
            sqlx::query("UPDATE vault_items SET why = ?1 WHERE id = ?2")
                .bind(why)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref status) = update.status {
            let s = serde_json::to_value(status).unwrap();
            sqlx::query("UPDATE vault_items SET status = ?1 WHERE id = ?2")
                .bind(s.as_str().unwrap())
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref priority) = update.priority {
            let p = serde_json::to_value(priority).unwrap();
            sqlx::query("UPDATE vault_items SET priority = ?1 WHERE id = ?2")
                .bind(p.as_str().unwrap())
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(pinned) = update.pinned {
            sqlx::query("UPDATE vault_items SET pinned = ?1 WHERE id = ?2")
                .bind(pinned as i32)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref tags) = update.tags {
            let tags_json = serde_json::to_string(tags).unwrap();
            sqlx::query("UPDATE vault_items SET tags = ?1 WHERE id = ?2")
                .bind(&tags_json)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_vault_item(id).await
    }

    async fn delete_vault_item(&self, id: Uuid) -> DomainResult<()> {
        let result = sqlx::query("DELETE FROM vault_items WHERE id = ?1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        if result.rows_affected() == 0 {
            return Err(DomainError::not_found("VaultItem", id.to_string()));
        }
        Ok(())
    }

    async fn stale_inbox_items(&self, hours: i64) -> DomainResult<Vec<VaultItem>> {
        let cutoff = (Utc::now() - chrono::Duration::hours(hours)).to_rfc3339();

        let rows = sqlx::query(
            "SELECT * FROM vault_items WHERE status = 'inbox' AND (last_opened_at IS NULL OR last_opened_at < ?1) AND created_at < ?1 ORDER BY created_at ASC"
        )
        .bind(&cutoff)
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
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                name: r.get("name"),
                color: r.get("color"),
                archived: r.get::<i32, _>("archived") != 0,
                created_at: parse_dt(r.get("created_at")),
                updated_at: parse_dt(r.get("updated_at")),
            })
            .collect())
    }

    async fn create_project(&self, input: CreateProject) -> DomainResult<Project> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query("INSERT INTO projects (id, name, color, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)")
            .bind(id.to_string())
            .bind(&input.name)
            .bind(&input.color)
            .bind(now.to_rfc3339())
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
        let id_str = id.to_string();

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE projects SET name = ?1 WHERE id = ?2")
                .bind(name)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(archived) = update.archived {
            sqlx::query("UPDATE projects SET archived = ?1 WHERE id = ?2")
                .bind(archived as i32)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        let row = sqlx::query("SELECT * FROM projects WHERE id = ?1")
            .bind(&id_str)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| DomainError::not_found("Project", id.to_string()))?;

        Ok(Project {
            id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
            name: row.get("name"),
            color: row.get("color"),
            archived: row.get::<i32, _>("archived") != 0,
            created_at: parse_dt(row.get("created_at")),
            updated_at: parse_dt(row.get("updated_at")),
        })
    }

    // ── Daily Logs ──

    async fn list_daily_logs(&self, filters: DailyLogFilters) -> DomainResult<Vec<DailyLog>> {
        let limit = filters.limit.unwrap_or(30);
        let offset = filters.offset.unwrap_or(0);

        let rows = sqlx::query(
            "SELECT * FROM daily_logs ORDER BY date DESC LIMIT ?1 OFFSET ?2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_daily_log).collect())
    }

    async fn get_daily_log_by_date(&self, date: NaiveDate) -> DomainResult<Option<DailyLog>> {
        let row = sqlx::query("SELECT * FROM daily_logs WHERE date = ?1")
            .bind(date.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(row.as_ref().map(row_to_daily_log))
    }

    async fn upsert_daily_log(&self, input: UpsertDailyLog) -> DomainResult<DailyLog> {
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        let date_str = input.date.to_string();
        let mood_str = input
            .mood
            .as_ref()
            .map(|m| serde_json::to_value(m).unwrap().as_str().unwrap().to_string());

        // Try insert, on conflict update
        sqlx::query(
            "INSERT INTO daily_logs (id, date, plan_day, cycles_completed, watch_done, build_done, prove_done, tab_limit, tab_limit_met, active_mission_id, focus_notes, sleep_hours_est, mood, created_at, updated_at)
             VALUES (?1, ?2, ?3, COALESCE(?4, 0), COALESCE(?5, 0), COALESCE(?6, 0), COALESCE(?7, 0), COALESCE(?8, 10), ?9, ?10, ?11, ?12, ?13, ?14, ?14)
             ON CONFLICT (date) DO UPDATE SET
                plan_day = COALESCE(excluded.plan_day, daily_logs.plan_day),
                cycles_completed = COALESCE(excluded.cycles_completed, daily_logs.cycles_completed),
                watch_done = COALESCE(excluded.watch_done, daily_logs.watch_done),
                build_done = COALESCE(excluded.build_done, daily_logs.build_done),
                prove_done = COALESCE(excluded.prove_done, daily_logs.prove_done)"
        )
        .bind(id.to_string())
        .bind(&date_str)
        .bind(input.plan_day)
        .bind(input.cycles_completed)
        .bind(input.watch_done.map(|b| b as i32))
        .bind(input.build_done.map(|b| b as i32))
        .bind(input.prove_done.map(|b| b as i32))
        .bind(input.tab_limit)
        .bind(input.tab_limit_met.map(|b| b as i32))
        .bind(input.active_mission_id.map(|u| u.to_string()))
        .bind(&input.focus_notes)
        .bind(input.sleep_hours_est)
        .bind(&mood_str)
        .bind(&now)
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
        let date_str = date.to_string();

        if let Some(cycles) = update.cycles_completed {
            sqlx::query("UPDATE daily_logs SET cycles_completed = ?1 WHERE date = ?2")
                .bind(cycles)
                .bind(&date_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(watch) = update.watch_done {
            sqlx::query("UPDATE daily_logs SET watch_done = ?1 WHERE date = ?2")
                .bind(watch as i32)
                .bind(&date_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(build) = update.build_done {
            sqlx::query("UPDATE daily_logs SET build_done = ?1 WHERE date = ?2")
                .bind(build as i32)
                .bind(&date_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(prove) = update.prove_done {
            sqlx::query("UPDATE daily_logs SET prove_done = ?1 WHERE date = ?2")
                .bind(prove as i32)
                .bind(&date_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_daily_log_by_date(date)
            .await?
            .ok_or_else(|| DomainError::not_found("DailyLog", date_str))
    }

    // ── Missions ──

    async fn list_missions(&self) -> DomainResult<Vec<Mission>> {
        let rows = sqlx::query("SELECT * FROM missions ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_mission).collect())
    }

    async fn get_mission(&self, id: Uuid) -> DomainResult<Mission> {
        let row = sqlx::query("SELECT * FROM missions WHERE id = ?1")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| DomainError::not_found("Mission", id.to_string()))?;

        Ok(row_to_mission(&row))
    }

    async fn create_mission(&self, input: CreateMission) -> DomainResult<Mission> {
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        let tab_limit = input.tab_limit.unwrap_or(10);
        let weekly_targets = input
            .weekly_targets
            .map(|wt| serde_json::to_string(&wt).unwrap());
        let kpis = input.kpis.map(|k| serde_json::to_string(&k).unwrap());

        sqlx::query("INSERT INTO missions (id, name, description, status, tab_limit, weekly_targets, kpis, created_at, updated_at) VALUES (?1, ?2, ?3, 'paused', ?4, ?5, ?6, ?7, ?7)")
            .bind(id.to_string())
            .bind(&input.name)
            .bind(&input.description)
            .bind(tab_limit)
            .bind(&weekly_targets)
            .bind(&kpis)
            .bind(&now)
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_mission(id).await
    }

    async fn update_mission(&self, id: Uuid, update: UpdateMission) -> DomainResult<Mission> {
        let id_str = id.to_string();

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE missions SET name = ?1 WHERE id = ?2")
                .bind(name)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(ref status) = update.status {
            let s = serde_json::to_value(status).unwrap();
            sqlx::query("UPDATE missions SET status = ?1 WHERE id = ?2")
                .bind(s.as_str().unwrap())
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_mission(id).await
    }

    async fn get_active_mission(&self) -> DomainResult<Option<Mission>> {
        let row = sqlx::query("SELECT * FROM missions WHERE status = 'active' LIMIT 1")
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
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE missions SET status = 'active', started_at = COALESCE(started_at, ?1) WHERE id = ?2")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_mission(id).await
    }

    // ── Courses ──

    async fn list_courses(&self) -> DomainResult<Vec<Course>> {
        let rows = sqlx::query("SELECT * FROM courses ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_course).collect())
    }

    async fn get_course(&self, id: Uuid) -> DomainResult<Course> {
        let row = sqlx::query("SELECT * FROM courses WHERE id = ?1")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| DomainError::not_found("Course", id.to_string()))?;

        Ok(row_to_course(&row))
    }

    async fn create_course(&self, input: CreateCourse) -> DomainResult<Course> {
        let id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();

        sqlx::query("INSERT INTO courses (id, name, provider, url, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, 'paused', ?5, ?5)")
            .bind(id.to_string())
            .bind(&input.name)
            .bind(&input.provider)
            .bind(&input.url)
            .bind(&now)
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_course(id).await
    }

    async fn update_course(&self, id: Uuid, update: UpdateCourse) -> DomainResult<Course> {
        let id_str = id.to_string();

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE courses SET name = ?1 WHERE id = ?2")
                .bind(name)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(minutes) = update.progress_minutes {
            sqlx::query("UPDATE courses SET progress_minutes = ?1 WHERE id = ?2")
                .bind(minutes)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_course(id).await
    }

    async fn get_active_course(&self) -> DomainResult<Option<Course>> {
        let row = sqlx::query("SELECT * FROM courses WHERE status = 'active' LIMIT 1")
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
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE courses SET status = 'active', started_at = COALESCE(started_at, ?1) WHERE id = ?2")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_course(id).await
    }

    async fn complete_course(&self, id: Uuid) -> DomainResult<Course> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE courses SET status = 'completed', completed_at = ?1 WHERE id = ?2")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_course(id).await
    }

    // ── Language Tracks ──

    async fn list_language_tracks(&self) -> DomainResult<Vec<LanguageTrack>> {
        let rows = sqlx::query("SELECT * FROM language_tracks ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(rows.iter().map(row_to_language_track).collect())
    }

    async fn get_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        let row = sqlx::query("SELECT * FROM language_tracks WHERE id = ?1")
            .bind(id.to_string())
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
        let now = Utc::now().to_rfc3339();
        let weekly_goal = input.weekly_goal_minutes.unwrap_or(420);

        sqlx::query("INSERT INTO language_tracks (id, name, status, weekly_goal_minutes, created_at, updated_at) VALUES (?1, ?2, 'paused', ?3, ?4, ?4)")
            .bind(id.to_string())
            .bind(&input.name)
            .bind(weekly_goal)
            .bind(&now)
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
        let id_str = id.to_string();

        if let Some(ref name) = update.name {
            sqlx::query("UPDATE language_tracks SET name = ?1 WHERE id = ?2")
                .bind(name)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }
        if let Some(minutes) = update.weekly_goal_minutes {
            sqlx::query("UPDATE language_tracks SET weekly_goal_minutes = ?1 WHERE id = ?2")
                .bind(minutes)
                .bind(&id_str)
                .execute(&self.pool)
                .await
                .map_err(map_db_err)?;
        }

        self.get_language_track(id).await
    }

    async fn get_active_language_track(&self) -> DomainResult<Option<LanguageTrack>> {
        let row = sqlx::query("SELECT * FROM language_tracks WHERE status = 'active' LIMIT 1")
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
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE language_tracks SET status = 'active', started_at = COALESCE(started_at, ?1) WHERE id = ?2")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_language_track(id).await
    }

    async fn complete_language_track(&self, id: Uuid) -> DomainResult<LanguageTrack> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE language_tracks SET status = 'completed', completed_at = ?1 WHERE id = ?2")
            .bind(&now)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        self.get_language_track(id).await
    }
}
