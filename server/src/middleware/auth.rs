use axum::{
    extract::{Request, State},
    middleware::Next,
};

use crate::{ApiResult, error::ErrorResponse, state::AppState};

#[axum::debug_middleware]
pub async fn authorize(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> ApiResult {
    let Some(authorization_header) = request.headers().get("Authorization") else {
        return Err(ErrorResponse::unauthorized());
    };
    let Ok(authorization_header_str) = authorization_header.to_str() else {
        return Err(ErrorResponse::unauthorized());
    };
    let uuid = state
        .token_service
        .validate_access_token(authorization_header_str, &state.config)?;
    request.extensions_mut().insert(uuid);
    Ok(next.run(request).await)
}
