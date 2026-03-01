-- FocusVault v2: Auth + Sync tables (Postgres)

-- ── Custom Types ──

CREATE TYPE sync_entity_type AS ENUM ('vault_item', 'mission', 'daily_log', 'course', 'language_track', 'project', 'settings');
CREATE TYPE sync_action AS ENUM ('create', 'update', 'delete');

-- ── Users ──

CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           TEXT NOT NULL,
    password_hash   TEXT NOT NULL,
    display_name    TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_users_email UNIQUE (email)
);

CREATE INDEX idx_users_email ON users(email);

-- ── API Keys ──

CREATE TABLE api_keys (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    key_hash        TEXT NOT NULL,
    key_prefix      TEXT NOT NULL,
    last_used_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_api_keys_user ON api_keys(user_id);
CREATE INDEX idx_api_keys_prefix ON api_keys(key_prefix);

-- ── Sync Events ──

CREATE TABLE sync_events (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entity_type     sync_entity_type NOT NULL,
    entity_id       UUID NOT NULL,
    action          sync_action NOT NULL,
    payload         JSONB NOT NULL DEFAULT '{}',
    device_id       TEXT NOT NULL,
    timestamp       TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sync_events_user_ts ON sync_events(user_id, timestamp);
CREATE INDEX idx_sync_events_device ON sync_events(device_id);
CREATE INDEX idx_sync_events_entity ON sync_events(entity_type, entity_id);

-- ── Triggers ──

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
