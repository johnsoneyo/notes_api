use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub aud: Option<String>,
    pub preferred_username: Option<String>,
}
