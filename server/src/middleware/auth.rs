use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    headers::{
        Authorization,
        authorization::{Basic, Bearer},
    },
    typed_header::TypedHeaderRejection,
};

use crate::{
    error::{AppResult, ClientError},
    state::AppState,
};

#[axum::debug_middleware]
pub async fn validate_jwt(
    State(state): State<AppState>,
    header_result: Result<TypedHeader<Authorization<Bearer>>, TypedHeaderRejection>,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let authorization_header = header_result.map_err(|_| ClientError::Unauthorized)?;
    let access_token = authorization_header.token();
    let claims = state
        .auth_service
        .validate_access_token(access_token)
        .map_err(|_| ClientError::Unauthorized)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[axum::debug_middleware]
pub async fn validate_basic_auth(
    State(state): State<AppState>,
    header_result: Result<TypedHeader<Authorization<Basic>>, TypedHeaderRejection>,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let authorization_header = header_result.map_err(|_| ClientError::Unauthorized)?;
    let user = state
        .user_service
        .find_user_by_email(authorization_header.username())
        .await?
        .ok_or(ClientError::Unauthorized)?;
    state
        .auth_service
        .verify_password(authorization_header.password(), &user.password_hash)
        .map_err(|_| ClientError::Unauthorized)?;
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
