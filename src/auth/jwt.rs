use crate::auth::jwks::Jwk;
use crate::auth::models::claims::Claims;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};

pub fn verify_jwt(
    token: &str,
    jwk: &Jwk,
    issuer: &str,
    audience: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);

    decode::<Claims>(token, &decoding_key, &validation)
}
