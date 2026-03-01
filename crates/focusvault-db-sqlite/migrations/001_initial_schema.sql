-- FocusVault v2: Initial Schema (SQLite)
-- SQLite doesn't have custom enum types, so we use TEXT with CHECK constraints.

-- ── Projects ──

CREATE TABLE IF NOT EXISTS projects (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    color       TEXT,
    archived    INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ── Vault Items ──

CREATE TABLE IF NOT EXISTS vault_items (
    id              TEXT PRIMARY KEY,
    item_type       TEXT NOT NULL CHECK (item_type IN ('link', 'note', 'snippet', 'file_ref')),
    url             TEXT,
    title           TEXT,
    favicon_url     TEXT,
    hostname        TEXT,
    why             TEXT NOT NULL,
    notes           TEXT,
    status          TEXT NOT NULL DEFAULT 'inbox' CHECK (status IN ('inbox', 'saved', 'actioned', 'archived')),
    priority        TEXT NOT NULL DEFAULT 'med' CHECK (priority IN ('low', 'med', 'high')),
    pinned          INTEGER NOT NULL DEFAULT 0,
    tags            TEXT NOT NULL DEFAULT '[]',  -- JSON array stored as text
    project_id      TEXT REFERENCES projects(id) ON DELETE SET NULL,
    source          TEXT NOT NULL DEFAULT 'manual' CHECK (source IN ('manual', 'extension', 'import', 'api')),
    due_at          TEXT,
    last_opened_at  TEXT,
    open_count      INTEGER NOT NULL DEFAULT 0,
    metadata        TEXT,  -- JSON stored as text
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_vault_items_status ON vault_items(status);
CREATE INDEX IF NOT EXISTS idx_vault_items_project ON vault_items(project_id);
CREATE INDEX IF NOT EXISTS idx_vault_items_created ON vault_items(created_at);
CREATE INDEX IF NOT EXISTS idx_vault_items_pinned ON vault_items(pinned) WHERE pinned = 1;

-- ── Missions ──

CREATE TABLE IF NOT EXISTS missions (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    description     TEXT,
    status          TEXT NOT NULL DEFAULT 'paused' CHECK (status IN ('active', 'paused', 'done')),
    started_at      TEXT,
    ended_at        TEXT,
    tab_limit       INTEGER NOT NULL DEFAULT 10,
    weekly_targets  TEXT,  -- JSON
    kpis            TEXT,  -- JSON
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ── Daily Logs ──

CREATE TABLE IF NOT EXISTS daily_logs (
    id                  TEXT PRIMARY KEY,
    date                TEXT NOT NULL UNIQUE,
    plan_day            INTEGER CHECK (plan_day IS NULL OR (plan_day >= 1 AND plan_day <= 30)),
    cycles_completed    INTEGER NOT NULL DEFAULT 0,
    watch_done          INTEGER NOT NULL DEFAULT 0,
    build_done          INTEGER NOT NULL DEFAULT 0,
    prove_done          INTEGER NOT NULL DEFAULT 0,
    tab_limit           INTEGER NOT NULL DEFAULT 10,
    tab_limit_met       INTEGER,  -- nullable boolean as 0/1
    active_mission_id   TEXT REFERENCES missions(id) ON DELETE SET NULL,
    focus_notes         TEXT,
    sleep_hours_est     REAL,
    mood                TEXT CHECK (mood IS NULL OR mood IN ('low', 'ok', 'high')),
    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ── Courses ──

CREATE TABLE IF NOT EXISTS courses (
    id                TEXT PRIMARY KEY,
    name              TEXT NOT NULL,
    provider          TEXT,
    url               TEXT,
    status            TEXT NOT NULL DEFAULT 'paused' CHECK (status IN ('active', 'paused', 'completed')),
    started_at        TEXT,
    completed_at      TEXT,
    progress_minutes  INTEGER NOT NULL DEFAULT 0,
    progress_notes    TEXT,
    created_at        TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at        TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ── Language Tracks ──

CREATE TABLE IF NOT EXISTS language_tracks (
    id                  TEXT PRIMARY KEY,
    name                TEXT NOT NULL,
    status              TEXT NOT NULL DEFAULT 'paused' CHECK (status IN ('active', 'paused', 'completed')),
    started_at          TEXT,
    completed_at        TEXT,
    weekly_goal_minutes INTEGER NOT NULL DEFAULT 420,
    notes               TEXT,
    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ── Updated at triggers ──

CREATE TRIGGER IF NOT EXISTS update_vault_items_updated_at AFTER UPDATE ON vault_items
BEGIN
    UPDATE vault_items SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_projects_updated_at AFTER UPDATE ON projects
BEGIN
    UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_missions_updated_at AFTER UPDATE ON missions
BEGIN
    UPDATE missions SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_daily_logs_updated_at AFTER UPDATE ON daily_logs
BEGIN
    UPDATE daily_logs SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_courses_updated_at AFTER UPDATE ON courses
BEGIN
    UPDATE courses SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_language_tracks_updated_at AFTER UPDATE ON language_tracks
BEGIN
    UPDATE language_tracks SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = NEW.id;
END;
