use chrono::Utc;
use std::time::Duration;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use jsonwebtoken::EncodingKey;
use jsonwebtoken::{Header, encode};
use serde::{Deserialize, Serialize};

use crate::{
    ApiResult, auth::password::verify_password, config::SERVER_CONFIG, error::ErrorResponse,
    services::PostgresService,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub fn router(postgres_service: PostgresService) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(postgres_service)
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(postgres_service): State<PostgresService>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
) -> ApiResult {
    let user = postgres_service
        .find_user_password_hash_by_email(authorization.username())
        .await?;
    verify_password(authorization.password(), &user.password_hash)?;

    let sub = user.uuid.to_string();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let access_token_expires_at =
        now + Duration::from_secs(SERVER_CONFIG.access_token_validity_secs);
    let iss = SERVER_CONFIG.issuer.clone();
    let secret = SERVER_CONFIG.secret.as_bytes();
    let key = EncodingKey::from_secret(secret);
    let exp = access_token_expires_at.timestamp() as usize;
    let access_token_claims = Claims { sub, exp, iat, iss };

    match encode(&Header::default(), &access_token_claims, &key) {
        Ok(access_token) => Ok((
            StatusCode::OK,
            Json(LoginResponse {
                access_token,
                token_type: String::from("Bearer"),
                expires_in: SERVER_CONFIG.access_token_validity_secs,
            }),
        )
            .into_response()),
        Err(_) => ErrorResponse::internal_server_error(),
    }
}
