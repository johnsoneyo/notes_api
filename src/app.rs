mod db;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:mysecretpassword@localhost:5432/notes_db")
            .await
            .expect("Failed to connect to database");

        Self { db }
    }
}