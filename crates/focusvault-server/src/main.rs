use std::sync::Arc;
use tracing_subscriber::EnvFilter;

mod app;
mod config;
mod error;
mod routes;
mod state;

use config::{Config, DbBackend};
use state::AppState;

#[tokio::main]
async fn main() {
    // Load .env file if present
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let bind_addr = config.bind_addr();

    tracing::info!("Initializing FocusVault server...");
    tracing::info!("Database: {:?}", config.db_backend);

    // Create repository based on database backend
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
    let router = app::create_router(state);

    tracing::info!("FocusVault server listening on {bind_addr}");
    tracing::info!("API docs: http://{bind_addr}/api/docs");

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, router)
        .await
        .expect("Server error");
}
