use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

pub async fn fetch_jwks(url: &str) -> Jwks {
    reqwest::get(url)
        .await
        .expect("failed request")
        .json::<Jwks>()
        .await
        .expect("invalid jwks")
}
