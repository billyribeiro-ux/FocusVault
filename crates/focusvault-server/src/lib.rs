pub mod app;
pub mod config;
pub mod error;
pub mod extractors;
pub mod middleware;
pub mod routes;
pub mod state;

use std::sync::Arc;

use axum::Router;
use config::{Config, DbBackend};
use state::AppState;

/// Create the full axum router from a Config.
/// Used by both the standalone server binary and the Tauri desktop app.
pub async fn build_router(config: Config) -> Router {
    let repo: Arc<dyn focusvault_core::repository::Repository> = match config.db_backend {
        DbBackend::Postgres => {
            let pool = focusvault_db_postgres::create_pool(&config.database_url)
                .await
                .expect("Failed to connect to Postgres");

            tracing::info!("Running Postgres migrations...");
            focusvault_db_postgres::run_migrations(&pool)
                .await
                .expect("Failed to run Postgres migrations");

            Arc::new(focusvault_db_postgres::repo::PostgresRepo::new(pool))
        }
        DbBackend::Sqlite => {
            let pool = focusvault_db_sqlite::create_pool(&config.database_url)
                .await
                .expect("Failed to connect to SQLite");

            tracing::info!("Running SQLite migrations...");
            focusvault_db_sqlite::run_migrations(&pool)
                .await
                .expect("Failed to run SQLite migrations");

            Arc::new(focusvault_db_sqlite::repo::SqliteRepo::new(pool))
        }
    };

    let state = AppState::new(config, repo);
    app::create_router(state)
}
