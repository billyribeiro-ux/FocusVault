use sqlx::PgPool;
use std::path::Path;

pub mod repo;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migration_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");

    let files = ["001_initial_schema.sql", "002_auth_sync.sql"];
    for file in &files {
        let sql = std::fs::read_to_string(migration_path.join(file))
            .unwrap_or_else(|_| panic!("Failed to read migration file: {file}"));
        sqlx::raw_sql(&sql).execute(pool).await?;
    }

    Ok(())
}
