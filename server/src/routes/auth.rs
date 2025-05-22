use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use std::time::Duration;
use uuid::Uuid;

use axum::{
    Extension, Json, Router, extract::Request, http::StatusCode, middleware::Next,
    response::IntoResponse, routing::post,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::{
    ApiResult,
    config::{Config, SERVER_CONFIG},
    error::ErrorResponse,
};

use super::user::hash_password;

#[derive(Serialize, Deserialize)]
pub struct Claims<'a> {
    pub sub: &'a str,
    pub iss: &'a str,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
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
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    Extension(pool): Extension<PgPool>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
) -> ApiResult {
    let mut transaction = pool.begin().await.unwrap();
    let user: GetUser = match sqlx::query_as(
        "SELECT uuid, name, password_hash FROM users WHERE name = $1 LIMIT 1;",
    )
    .bind(authorization.username())
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(user) => user,
        Err(_) => {
            return ErrorResponse::unauthorized();
        }
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default()
        .verify_password(authorization.password().as_bytes(), &parsed_hash)
        .is_err()
    {
        return ErrorResponse::unauthorized();
    }

    sqlx::query("DELETE FROM refresh_tokens WHERE user_uuid = $1;")
        .bind(user.uuid)
        .execute(&mut *transaction)
        .await
        .unwrap();

    let user_uuid = user.uuid.to_string();
    let sub = user_uuid.as_str();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let refresh_token_expires_at =
        now + Duration::from_secs(SERVER_CONFIG.refresh_token_validity_secs);
    let exp = refresh_token_expires_at.timestamp() as usize;
    let iss = &SERVER_CONFIG.issuer;
    let refresh_token_claims = Claims { sub, iss, exp, iat };

    let secret = SERVER_CONFIG.secret.as_bytes();
    let key = EncodingKey::from_secret(secret);

    let refresh_token = encode(&Header::default(), &refresh_token_claims, &key).unwrap();
    let refresh_token_hash = hash_password(&refresh_token);

    sqlx::query(
        "INSERT INTO refresh_tokens (
            uuid,
            user_uuid,
            token_hash,
            created_at
        ) VALUES (
            $1, $2, $3, $4
        )",
    )
    .bind(Uuid::now_v7())
    .bind(user.uuid)
    .bind(&refresh_token_hash)
    .bind(Utc::now())
    .execute(&mut *transaction)
    .await
    .unwrap();

    let access_token_expires_at =
        now + Duration::from_secs(SERVER_CONFIG.access_token_validity_secs);
    let exp = access_token_expires_at.timestamp() as usize;
    let access_token_claims = Claims { sub, exp, iat, iss };

    match encode(&Header::default(), &access_token_claims, &key) {
        Ok(access_token) => {
            transaction.commit().await.unwrap();
            Ok((
                StatusCode::OK,
                Json(LoginResponse {
                    access_token,
                    refresh_token,
                    token_type: String::from("Bearer"),
                    expires_in: SERVER_CONFIG.access_token_validity_secs,
                }),
            )
                .into_response())
        }
        Err(_) => ErrorResponse::internal_server_error(),
    }
}

#[axum::debug_handler]
pub async fn refresh(
    Extension(_pool): Extension<PgPool>,
    Extension(_config): Extension<Config>,
) -> ApiResult {
    ErrorResponse::internal_server_error()
}

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
