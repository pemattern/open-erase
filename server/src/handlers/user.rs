use axum::{
    Extension, Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{AppResult, ClientError},
    schemas::user::{
        ServerDeleteUserResponse, ServerGetUserResponse, ServerPatchUserRequest,
        ServerPatchUserResponse, ServerPostUserRequest, ServerPostUserResponse,
    },
    services::auth::Claims,
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<ServerGetUserResponse> {
    let user = state
        .user_service
        .find_user_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<ServerGetUserResponse> {
    let id = Uuid::parse_str(&claims.sub).unwrap();
    let user = state
        .user_service
        .find_user_by_id(id)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    Json(user): Json<ServerPostUserRequest>,
) -> AppResult<ServerPostUserResponse> {
    let user = state.user_service.create_user(user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn patch_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(user): Json<ServerPatchUserRequest>,
) -> AppResult<ServerPatchUserResponse> {
    let user = state.user_service.update_user(id, user).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<ServerDeleteUserResponse> {
    let user = state.user_service.delete_user(id).await?;
    Ok(user.into())
}
