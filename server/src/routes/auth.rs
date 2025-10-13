use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};

use crate::{AppResult, error::ClientError, schemas::token::TokenResponse, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
) -> AppResult<impl IntoResponse> {
    let user = state
        .database_service
        .find_user_by_email(authorization.username())
        .await?
        .ok_or(ClientError::NotFound)?;
    state
        .hashing_service
        .verify_password(authorization.password(), &user.password_hash)?;
    let access_token = state
        .token_service
        .generate_access_token(user.id, &state.config)?;
    Ok(Json(TokenResponse {
        access_token,
        token_type: String::from("Bearer"),
        expires_in: state.config.access_token_validity_secs,
    })
    .into_response())
}
