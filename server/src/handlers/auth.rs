use axum::{Extension, extract::State};

use crate::{error::AppResult, models::User, schemas::token::LoginResponse, state::AppState};

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> AppResult<LoginResponse> {
    let access_token = state.auth_service.generate_access_token(&user)?;
    let refresh_token = state.auth_service.generate_refresh_token(&user).await?;
    Ok(LoginResponse {
        access_token,
        refresh_token,
        token_type: String::from("Bearer"),
        expires_in: 60 * 15, // Todo get from auth service
    })
}
