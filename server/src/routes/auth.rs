use axum::{Extension, Router, extract::State, routing::post};

use crate::{error::AppResult, models::User, schemas::token::TokenResponse, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/auth/login")]
pub async fn login(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> AppResult<TokenResponse> {
    let access_token = state.token_service.generate(user.id, &state.config)?;
    Ok(TokenResponse {
        access_token,
        token_type: String::from("Bearer"),
        expires_in: state.config.access_token_validity_secs,
    })
}
