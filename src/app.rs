use sqlx::postgres::PgPoolOptions;

use crate::state;

use state::config::Config;
use state::db::DbState;
use state::http::HttpState;

use crate::auth::jwks::Jwks;
use crate::auth::jwks::fetch_jwks;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub config: Config,
    pub http: HttpState,
    pub issuer: String,
    pub audience: String,
    pub jwks: Arc<RwLock<Jwks>>,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .expect("PORT must be a number");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("DB connection failed");

        let issuer = std::env::var("KEYCLOAK_ISSUER").expect("missing issuer");

        let audience = std::env::var("KEYCLOAK_AUDIENCE").expect("missing audience");

        let jwks_url = format!("{}/protocol/openid-connect/certs", issuer);

        let jwks = fetch_jwks(&jwks_url).await;

        Self {
            db: DbState { pool: pool },
            config: Config { database_url, port },
            http: HttpState {
                client: reqwest::Client::new(),
            },
            issuer,
            audience,
            jwks: Arc::new(RwLock::new(jwks)),
        }
    }
}
