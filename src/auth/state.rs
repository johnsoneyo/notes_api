use crate::auth::jwks::Jwks;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AuthState {
    pub issuer: String,
    pub audience: String,
    pub jwks: Arc<RwLock<Jwks>>,
}
