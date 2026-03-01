-- FocusVault v2: Initial Schema (Postgres)

-- ── Custom Types ──

CREATE TYPE vault_item_type AS ENUM ('link', 'note', 'snippet', 'file_ref');
CREATE TYPE vault_item_status AS ENUM ('inbox', 'saved', 'actioned', 'archived');
CREATE TYPE priority AS ENUM ('low', 'med', 'high');
CREATE TYPE capture_source AS ENUM ('manual', 'extension', 'import', 'api');
CREATE TYPE mission_status AS ENUM ('active', 'paused', 'done');
CREATE TYPE course_status AS ENUM ('active', 'paused', 'completed');
CREATE TYPE language_track_status AS ENUM ('active', 'paused', 'completed');
CREATE TYPE mood AS ENUM ('low', 'ok', 'high');

-- ── Projects ──

CREATE TABLE projects (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    color       TEXT,
    archived    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_project_name UNIQUE (name)
);

-- ── Vault Items ──

CREATE TABLE vault_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_type       vault_item_type NOT NULL,
    url             TEXT,
    title           TEXT,
    favicon_url     TEXT,
    hostname        TEXT,
    why             TEXT NOT NULL,
    notes           TEXT,
    status          vault_item_status NOT NULL DEFAULT 'inbox',
    priority        priority NOT NULL DEFAULT 'med',
    pinned          BOOLEAN NOT NULL DEFAULT FALSE,
    tags            TEXT[] NOT NULL DEFAULT '{}',
    project_id      UUID REFERENCES projects(id) ON DELETE SET NULL,
    source          capture_source NOT NULL DEFAULT 'manual',
    due_at          TIMESTAMPTZ,
    last_opened_at  TIMESTAMPTZ,
    open_count      INTEGER NOT NULL DEFAULT 0,
    metadata        JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Indexes for common queries
CREATE INDEX idx_vault_items_status ON vault_items(status);
CREATE INDEX idx_vault_items_project ON vault_items(project_id);
CREATE INDEX idx_vault_items_created ON vault_items(created_at DESC);
CREATE INDEX idx_vault_items_due ON vault_items(due_at) WHERE due_at IS NOT NULL;
CREATE INDEX idx_vault_items_pinned ON vault_items(pinned) WHERE pinned = TRUE;

-- Full-text search index
CREATE INDEX idx_vault_items_search ON vault_items
    USING gin(to_tsvector('english', coalesce(title, '') || ' ' || coalesce(why, '') || ' ' || coalesce(notes, '')));

-- ── Missions ──

CREATE TABLE missions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    description     TEXT,
    status          mission_status NOT NULL DEFAULT 'paused',
    started_at      TIMESTAMPTZ,
    ended_at        TIMESTAMPTZ,
    tab_limit       INTEGER NOT NULL DEFAULT 10,
    weekly_targets  JSONB,
    kpis            JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Enforce single active mission at DB level
CREATE UNIQUE INDEX uq_missions_single_active
    ON missions ((TRUE)) WHERE status = 'active';

-- ── Daily Logs ──

CREATE TABLE daily_logs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date                DATE NOT NULL,
    plan_day            INTEGER CHECK (plan_day IS NULL OR (plan_day >= 1 AND plan_day <= 30)),
    cycles_completed    INTEGER NOT NULL DEFAULT 0,
    watch_done          BOOLEAN NOT NULL DEFAULT FALSE,
    build_done          BOOLEAN NOT NULL DEFAULT FALSE,
    prove_done          BOOLEAN NOT NULL DEFAULT FALSE,
    tab_limit           INTEGER NOT NULL DEFAULT 10,
    tab_limit_met       BOOLEAN,
    active_mission_id   UUID REFERENCES missions(id) ON DELETE SET NULL,
    focus_notes         TEXT,
    sleep_hours_est     DOUBLE PRECISION,
    mood                mood,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_daily_log_date UNIQUE (date)
);

-- ── Courses ──

CREATE TABLE courses (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name              TEXT NOT NULL,
    provider          TEXT,
    url               TEXT,
    status            course_status NOT NULL DEFAULT 'paused',
    started_at        TIMESTAMPTZ,
    completed_at      TIMESTAMPTZ,
    progress_minutes  INTEGER NOT NULL DEFAULT 0,
    progress_notes    TEXT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Enforce single active course at DB level
CREATE UNIQUE INDEX uq_courses_single_active
    ON courses ((TRUE)) WHERE status = 'active';

-- ── Language Tracks ──

CREATE TABLE language_tracks (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                TEXT NOT NULL,
    status              language_track_status NOT NULL DEFAULT 'paused',
    started_at          TIMESTAMPTZ,
    completed_at        TIMESTAMPTZ,
    weekly_goal_minutes INTEGER NOT NULL DEFAULT 420,
    notes               TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Enforce single active language track at DB level
CREATE UNIQUE INDEX uq_language_tracks_single_active
    ON language_tracks ((TRUE)) WHERE status = 'active';

-- ── Updated at trigger ──

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_vault_items_updated_at
    BEFORE UPDATE ON vault_items FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE ON projects FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_missions_updated_at
    BEFORE UPDATE ON missions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_daily_logs_updated_at
    BEFORE UPDATE ON daily_logs FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_courses_updated_at
    BEFORE UPDATE ON courses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_language_tracks_updated_at
    BEFORE UPDATE ON language_tracks FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
