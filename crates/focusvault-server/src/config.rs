use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub frontend_url: String,
    pub db_backend: DbBackend,
}

#[derive(Debug, Clone)]
pub enum DbBackend {
    Postgres,
    Sqlite,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:focusvault.db?mode=rwc".into());

        let db_backend = if database_url.starts_with("postgres") {
            DbBackend::Postgres
        } else {
            DbBackend::Sqlite
        };

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("PORT must be a number"),
            database_url,
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".into()),
            db_backend,
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
