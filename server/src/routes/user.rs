use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use uuid::Uuid;

use crate::{ApiResult, middleware::auth, schemas::user::CreateUserRequest, state::AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/{uuid}", get(get_user).delete(delete_user))
        .route("/", post(post_user))
        .layer(middleware::from_fn_with_state(state, auth::authorize))
}

#[axum::debug_handler]
pub async fn get_user(State(state): State<AppState>, Path(uuid): Path<Uuid>) -> Response {
    match state.database_service.find_user_by_uuid(uuid).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[axum::debug_handler]
pub async fn post_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUserRequest>,
) -> ApiResult {
    let password_hash = state.hashing_service.hash_password(&user.password)?;
    state
        .database_service
        .create_user(user.email, password_hash)
        .await?;
    Ok(StatusCode::CREATED.into_response())
}

#[axum::debug_handler]
pub async fn delete_user(State(state): State<AppState>, Path(uuid): Path<Uuid>) -> ApiResult {
    state.database_service.delete_user(uuid).await?;
    Ok(StatusCode::OK.into_response())
}
