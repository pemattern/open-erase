use axum::{Extension, extract::State};

use crate::{error::AppResult, models::User, schemas::token::TokenResponse, state::AppState};

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> AppResult<TokenResponse> {
    let access_token = state
        .auth_service
        .generate_access_token(user, &state.config)?;
    Ok(TokenResponse {
        access_token,
        token_type: String::from("Bearer"),
        expires_in: state.config.access_token_validity_secs,
    })
}
