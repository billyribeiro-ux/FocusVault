// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use focusvault_server::config::Config;
use tauri::Manager;

/// Tauri command: returns the local API server URL so the frontend knows where to connect.
#[tauri::command]
fn get_api_url(app: tauri::AppHandle) -> String {
    app.state::<ApiUrl>().0.clone()
}

struct ApiUrl(String);

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("focusvault=info".parse().unwrap()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Determine a data directory for the SQLite database
            let app_data = app
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data directory");
            std::fs::create_dir_all(&app_data).ok();

            let db_path = app_data.join("focusvault.db");
            let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

            tracing::info!("Database path: {}", db_path.display());

            // Start the embedded API server on a random port
            let jwt_secret = {
                use rand::Rng;
                rand::rng()
                    .sample_iter(&rand::distr::Alphanumeric)
                    .take(64)
                    .map(char::from)
                    .collect()
            };

            let config = Config {
                host: "127.0.0.1".into(),
                port: 0, // OS assigns a free port
                database_url: db_url,
                frontend_url: "tauri://localhost".into(),
                db_backend: focusvault_server::config::DbBackend::Sqlite,
                jwt_secret,
            };

            let handle = app.handle().clone();

            // Spawn the server in a background tokio runtime
            tauri::async_runtime::spawn(async move {
                let router = focusvault_server::build_router(config).await;

                let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                    .await
                    .expect("Failed to bind API server");

                let addr = listener.local_addr().unwrap();
                let api_url = format!("http://{addr}");
                tracing::info!("Embedded API server listening on {api_url}");

                // Store the URL so the frontend can discover it
                handle.manage(ApiUrl(api_url.clone()));

                // Emit event to the frontend with the API URL
                let _ = handle.emit("api-ready", &api_url);

                axum::serve(listener, router)
                    .await
                    .expect("API server error");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_api_url])
        .run(tauri::generate_context!())
        .expect("error while running FocusVault desktop");
}
