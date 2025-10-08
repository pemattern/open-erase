use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};

use crate::{ApiResult, schemas::token::TokenResponse, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new().route("/login", post(login)).with_state(state)
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    TypedHeader(authorization): TypedHeader<Authorization<Basic>>,
) -> ApiResult {
    let user = state
        .postgres_service
        .find_user_password_hash_by_email(authorization.username())
        .await?;
    state
        .hashing_service
        .verify_password(authorization.password(), &user.password_hash)?;
    let access_token = state
        .token_service
        .generate_access_token(user.uuid, &state.config)?;
    Ok(Json(TokenResponse {
        access_token,
        token_type: String::from("Bearer"),
        expires_in: state.config.access_token_validity_secs,
    })
    .into_response())
}
