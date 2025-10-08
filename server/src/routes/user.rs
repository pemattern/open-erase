use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, patch},
};
use uuid::Uuid;

use crate::{
    ApiResult,
    middleware::auth,
    schemas::user::{CreateUserRequest, UpdateUserPasswordRequest},
    state::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user", get(get_user).post(post_user).delete(delete_user))
        .route("/user/update-password", patch(update_password))
        .layer(middleware::from_fn_with_state(state, auth::authorize))
}

#[axum::debug_handler]
pub async fn get_user(State(state): State<AppState>, Extension(uuid): Extension<Uuid>) -> Response {
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
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(uuid): Extension<Uuid>,
) -> ApiResult {
    state.database_service.delete_user(uuid).await?;
    Ok(StatusCode::OK.into_response())
}

#[axum::debug_handler]
pub async fn update_password(
    State(state): State<AppState>,
    Extension(uuid): Extension<Uuid>,
    Json(user): Json<UpdateUserPasswordRequest>,
) -> ApiResult {
    let password_hash = state.hashing_service.hash_password(&user.password)?;
    state
        .database_service
        .update_user_password(uuid, password_hash)
        .await?;
    Ok(StatusCode::OK.into_response())
}
