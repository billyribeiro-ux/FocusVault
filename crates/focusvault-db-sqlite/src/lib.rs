use sqlx::{AssertSqlSafe, SqlitePool};
use std::path::Path;

pub mod repo;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Ensure the URL has the sqlite: prefix
    let url = if database_url.starts_with("sqlite:") {
        database_url.to_string()
    } else {
        format!("sqlite:{database_url}?mode=rwc")
    };

    SqlitePool::connect(&url).await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Enable WAL mode for better concurrent read performance
    sqlx::query("PRAGMA journal_mode=WAL").execute(pool).await?;

    sqlx::query("PRAGMA foreign_keys=ON").execute(pool).await?;

    let migration_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");

    let files = ["001_initial_schema.sql", "002_auth_sync.sql"];
    for file in &files {
        let sql = std::fs::read_to_string(migration_path.join(file))
            .unwrap_or_else(|_| panic!("Failed to read migration file: {file}"));
        sqlx::raw_sql(AssertSqlSafe(sql)).execute(pool).await?;
    }

    Ok(())
}
