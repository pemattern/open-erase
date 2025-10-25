use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
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

pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

#[axum::debug_middleware]
pub async fn validate_access_token(
    State(state): State<AppState>,
    header_result: Result<TypedHeader<Authorization<Bearer>>, TypedHeaderRejection>,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let authorization_header = header_result.map_err(|_| ClientError::Unauthorized)?;
    let access_token = authorization_header.token();
    let claims = state
        .auth_service
        .get_valid_access_token_claims(access_token)
        .ok_or(ClientError::Unauthorized)?;
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[axum::debug_middleware]
pub async fn validate_refresh_token(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let refresh_token_cookie = jar
        .get(REFRESH_TOKEN_COOKIE)
        .ok_or(ClientError::Unauthorized)?;
    let refresh_token = state
        .auth_service
        .get_valid_refresh_token(refresh_token_cookie.value())
        .await?
        .ok_or(ClientError::Unauthorized)?;
    request.extensions_mut().insert(refresh_token);
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
        .auth_service
        .get_user_from_basic_auth(
            authorization_header.username(),
            authorization_header.password(),
        )
        .await?
        .ok_or(ClientError::Unauthorized)?;
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
