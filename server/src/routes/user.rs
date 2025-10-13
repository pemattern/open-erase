use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppResult,
    error::ClientError,
    middleware::auth,
    schemas::user::{CreateUserRequest, UserResponse},
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
) -> AppResult<UserResponse> {
    let user = state.database_service.find_user_by_uuid(uuid).await?;
    let response = user.ok_or(ClientError::NotFound)?;
    Ok(response)
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUserRequest>,
) -> AppResult {
    let password_hash = state.hashing_service.hash_password(&user.password)?;
    state
        .database_service
        .create_user(user.email, password_hash)
        .await?;
    Ok(StatusCode::CREATED.into_response())
}

#[axum::debug_handler]
pub async fn delete_user(State(state): State<AppState>, Path(uuid): Path<Uuid>) -> AppResult {
    state.database_service.delete_user(uuid).await?;
    Ok(StatusCode::OK.into_response())
}
