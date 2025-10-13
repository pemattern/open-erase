use axum::{Router, extract::State, routing::post};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
    typed_header::TypedHeaderRejection,
};

use crate::{AppResult, error::ClientError, schemas::token::TokenResponse, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    authorization_header: Result<TypedHeader<Authorization<Basic>>, TypedHeaderRejection>,
) -> AppResult<TokenResponse> {
    let authorization_header_value = authorization_header.map_err(|_| ClientError::Unauthorized)?;
    let user = state
        .database_service
        .find_user_by_email(authorization_header_value.username())
        .await?
        .ok_or(ClientError::Unauthorized)?;
    state
        .hashing_service
        .verify_password(authorization_header_value.password(), &user.password_hash)
        .map_err(|_| ClientError::Unauthorized)?;
    let access_token = state
        .token_service
        .generate_access_token(user.id, &state.config)?;
    Ok(TokenResponse {
        access_token,
        token_type: String::from("Bearer"),
        expires_in: state.config.access_token_validity_secs,
    })
}
