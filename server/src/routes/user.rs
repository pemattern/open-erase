use axum::{
    Json, Router,
    extract::{Path, State},
    middleware,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppResult,
    error::ClientError,
    middleware::auth,
    schemas::user::{DeleteUserResponse, GetUserResponse, PostUserRequest, PostUserResponse},
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/{uuid}", get(get_user).delete(delete_user))
        .route("/", post(post_user))
        .layer(middleware::from_fn_with_state(state, auth::authorize))
}

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
