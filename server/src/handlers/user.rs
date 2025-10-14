use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::{AppResult, ClientError},
    schemas::user::{DeleteUserResponse, GetUserResponse, PostUserRequest, PostUserResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<GetUserResponse> {
    let user = state
        .database_service
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
    let password_hash = state.hashing_service.hash_password(&user.password)?;
    let user = state
        .database_service
        .create_user(user.email, password_hash)
        .await?;
    Ok(user.into())
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> AppResult<DeleteUserResponse> {
    let user = state.database_service.delete_user(uuid).await?;
    Ok(user.into())
}
