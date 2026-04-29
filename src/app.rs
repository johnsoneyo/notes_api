
use sqlx::postgres::PgPoolOptions;

use crate::state;

use state::config::Config;
use state::db::DbState;
use state::http::HttpState;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub config: Config,
    pub http: HttpState,
}


impl AppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not set");

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .expect("PORT must be a number");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("DB connection failed");

        Self {
            db: DbState { pool: pool},
            config: Config {
                database_url,
                port
            },
            http: HttpState {
                client: reqwest::Client::new(),
            }
        }
    }
}