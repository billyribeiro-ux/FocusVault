use focusvault_server::config::Config;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let bind_addr = config.bind_addr();

    tracing::info!("Initializing FocusVault server...");
    tracing::info!("Database: {:?}", config.db_backend);

    let router = focusvault_server::build_router(config).await;

    tracing::info!("FocusVault server listening on {bind_addr}");
    tracing::info!("API docs: http://{bind_addr}/api/docs");

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, router)
        .await
        .expect("Server error");
}
