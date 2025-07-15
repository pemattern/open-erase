use axum::{Extension, extract::Request, middleware::Next};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ApiResult, config::Config, error::ErrorResponse};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

#[axum::debug_middleware]
pub async fn authorize(
    Extension(config): Extension<Config>,
    mut request: Request,
    next: Next,
) -> ApiResult {
    let secret = config.secret;
    let key = DecodingKey::from_secret(secret.as_bytes());
    let issuer = config.issuer;
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[issuer]);

    let authorization_header = match request.headers().get("Authorization") {
        Some(v) => v,
        None => return ErrorResponse::unauthorized(),
    };

    let authorization = match authorization_header.to_str() {
        Ok(v) => v,
        Err(_) => return ErrorResponse::unauthorized(),
    };

    if !authorization.starts_with("Bearer ") {
        return ErrorResponse::unauthorized();
    };

    let jwt = authorization.trim_start_matches("Bearer ");

    let claims = match decode::<Claims>(jwt, &key, &Validation::new(jsonwebtoken::Algorithm::HS256))
    {
        Ok(token_data) => token_data.claims,
        Err(_) => return ErrorResponse::unauthorized(),
    };

    let user = match Uuid::parse_str(&claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return ErrorResponse::unauthorized(),
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
