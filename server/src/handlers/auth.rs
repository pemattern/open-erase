use axum::{Extension, extract::State};

use crate::{
    error::AppResult,
    models::User,
    schemas::token::{LoginResponse, RefreshResponse},
    state::AppState,
};

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> AppResult<LoginResponse> {
    let access_token = state.auth_service.generate_access_token(&user)?;
    let refresh_token = state.auth_service.generate_refresh_token(&user).await?;
    Ok(LoginResponse::new(access_token, refresh_token))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/refresh")]
pub async fn refresh(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> AppResult<RefreshResponse> {
    let access_token = state.auth_service.generate_access_token(&user)?;
    Ok(RefreshResponse::new(access_token))
}
