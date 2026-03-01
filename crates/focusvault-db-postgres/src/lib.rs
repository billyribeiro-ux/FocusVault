use sqlx::PgPool;
use std::path::Path;

pub mod repo;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Run migration files from the migrations directory
    let migration_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");

    // Read and execute migration SQL
    let sql = std::fs::read_to_string(migration_path.join("001_initial_schema.sql"))
        .expect("Failed to read migration file");

    sqlx::raw_sql(&sql).execute(pool).await?;

    Ok(())
}
