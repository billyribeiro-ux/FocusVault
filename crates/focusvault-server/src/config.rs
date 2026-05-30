use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub frontend_url: String,
    pub db_backend: DbBackend,
    pub jwt_secret: String,
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

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET not set — using random ephemeral secret. Tokens will not survive restarts.");
            use rand::RngExt;
            let secret: String = rand::rng()
                .sample_iter(&rand::distr::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();
            secret
        });

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
            jwt_secret,
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
