use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};

use crate::{AppResult, error::ClientError, state::AppState};

#[axum::debug_middleware]
pub async fn authorize(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let authorization_header = request
        .headers()
        .get("Authorization")
        .ok_or(ClientError::Unauthorized)?;
    let authorization_header_value = authorization_header
        .to_str()
        .map_err(|_| ClientError::Unauthorized)?;

    // TODO use Claims as extension, not uuid

    let uuid = state
        .token_service
        .validate_access_token(authorization_header_value, &state.config)?;
    request.extensions_mut().insert(uuid);
    Ok(next.run(request).await)
}
