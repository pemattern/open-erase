use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use std::time::Duration;
use uuid::Uuid;

use axum::{
    Extension, Json, Router,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as};

#[derive(Clone)]
pub struct Config {
    pub secret: String,
    pub issuer: String,
    pub validity_secs: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetJWT {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct GetUser {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
}

pub fn router() -> Router {
    Router::new().route("/token", get(new))
}

#[axum::debug_handler]
pub async fn new(
    Extension(pool): Extension<PgPool>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
) -> Response {
    let config = Config {
        secret: "abc123".to_string(),
        issuer: "me".to_string(),
        validity_secs: 3600,
    };
    let user = match query_as::<_, GetUser>(
        "SELECT uuid, name, password_hash FROM users WHERE name = $1 LIMIT 1",
    )
    .bind(authorization.username())
    .fetch_one(&pool)
    .await
    {
        Ok(user) => user,
        Err(_) => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default()
        .verify_password(authorization.password().as_bytes(), &parsed_hash)
        .is_err()
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let sub = user.uuid.to_string();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_at = now + Duration::from_secs(config.validity_secs);
    let exp = expires_at.timestamp() as usize;
    let iss = config.issuer;
    let claims = Claims { sub, exp, iat, iss };

    let secret = config.secret;
    let key = EncodingKey::from_secret(secret.as_bytes());

    match encode(&Header::default(), &claims, &key) {
        Ok(jwt) => (
            StatusCode::OK,
            Json(GetJWT {
                access_token: jwt,
                token_type: String::from("Bearer"),
                expires_in: config.validity_secs,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn authorize(
    Extension(config): Extension<Config>,
    mut request: Request,
    next: Next,
) -> Response {
    let secret = config.secret;
    let key = DecodingKey::from_secret(secret.as_bytes());
    let issuer = config.issuer;
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[issuer]);

    let authorization_header = match request.headers().get("Authorization") {
        Some(v) => v,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let authorization = match authorization_header.to_str() {
        Ok(v) => v,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if !authorization.starts_with("Bearer ") {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let jwt = authorization.trim_start_matches("Bearer ");

    let claims = match decode::<Claims>(jwt, &key, &Validation::new(jsonwebtoken::Algorithm::HS256))
    {
        Ok(token_data) => token_data.claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let user = match Uuid::parse_str(&claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    request.extensions_mut().insert(user);
    next.run(request).await
}
