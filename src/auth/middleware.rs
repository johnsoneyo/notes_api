use crate::app::AppState;
use crate::auth::jwt::verify_jwt;
use axum::response::IntoResponse;
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::decode_header;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let auth_header = match req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        Some(h) => h,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let header = match decode_header(token) {
        Ok(h) => h,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let kid = match header.kid {
        Some(k) => k,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let jwks = state.jwks.read().await;

    let jwk = match jwks.keys.iter().find(|k| k.kid == kid) {
        Some(k) => k,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let claims = match verify_jwt(token, jwk, &state.issuer, &state.audience) {
        Ok(data) => data.claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(claims);

    next.run(req).await
}
