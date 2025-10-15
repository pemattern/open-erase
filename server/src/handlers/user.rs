use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{AppResult, ClientError},
    schemas::user::{
        DeleteUserResponse, GetUserResponse, PatchUserRequest, PatchUserResponse, PostUserRequest,
        PostUserResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<GetUserResponse> {
    let user = state
        .user_service
        .find_user_by_id(uuid)
        .await?
        .ok_or(ClientError::NotFound)?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    Json(user): Json<PostUserRequest>,
) -> AppResult<PostUserResponse> {
    let password_hash = state.auth_service.hash_password(&user.password)?;
    let user = state
        .user_service
        .create_user(user.email, password_hash)
        .await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn patch_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(user): Json<PatchUserRequest>,
) -> AppResult<PatchUserResponse> {
    let user = state.user_service.update_user(uuid, user.email).await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<DeleteUserResponse> {
    let user = state.user_service.delete_user(uuid).await?;
    Ok(user.into())
}
