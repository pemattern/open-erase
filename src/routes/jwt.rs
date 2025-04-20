use chrono::{DateTime, Local, Utc};
use std::time::Duration;
use uuid::Uuid;

use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};

#[derive(Clone)]
pub struct Config {
    pub secret: String,
    pub issuer: String,
    pub validity_duration: u64,
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
    pub uuid: String,
    pub name: String,
    pub password: String,
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
        validity_duration: 3600,
    };
    let sub =
        match sqlx::query_as::<_, GetUser>("SELECT * FROM users WHERE name = $1 AND password = $2")
            .bind(authorization.username())
            .bind(authorization.password())
            .fetch_one(&pool)
            .await
        {
            Ok(user) => user.uuid,
            Err(_) => {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        };

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_at = now + Duration::from_secs(config.validity_duration);
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
                expires_in: config.validity_duration,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
