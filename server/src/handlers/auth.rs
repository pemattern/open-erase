use axum::{Extension, extract::State};

use crate::{
    error::AppResult,
    models::{RefreshToken, User},
    schemas::token::{ServerLoginResponse, ServerLogoutResponse, ServerRefreshResponse},
    state::AppState,
};

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> AppResult<ServerLoginResponse> {
    let access_token = state.auth_service.generate_access_token(user.id)?;
    let refresh_token = state
        .auth_service
        .generate_refresh_token_from_login(user.id)
        .await?;
    Ok(ServerLoginResponse::new(access_token, refresh_token))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/refresh")]
pub async fn refresh(
    State(state): State<AppState>,
    Extension(refresh_token): Extension<RefreshToken>,
) -> AppResult<ServerRefreshResponse> {
    let access_token = state
        .auth_service
        .generate_access_token(refresh_token.user_id)?;
    let new_refresh_token = state
        .auth_service
        .cycle_refresh_token(&refresh_token)
        .await?;
    Ok(ServerRefreshResponse::new(access_token, new_refresh_token))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/logout")]
pub async fn logout(
    State(state): State<AppState>,
    Extension(refresh_token): Extension<RefreshToken>,
) -> AppResult<ServerLogoutResponse> {
    let _ = state
        .auth_service
        .mark_refresh_token_as_used(&refresh_token)
        .await?;
    Ok(ServerLogoutResponse)
}
