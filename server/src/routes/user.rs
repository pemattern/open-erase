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
    middleware::auth,
    schemas::user::{CreateUserRequest, UpdateUserPasswordRequest},
    services::PostgresService,
};

pub fn router(postgres_service: PostgresService) -> Router {
    Router::new()
        .route("/user", get(get_user).post(post_user).delete(delete_user))
        .route("/user/update-password", patch(update_password))
        .layer(middleware::from_fn(auth::authorize))
        .with_state(postgres_service)
}

#[axum::debug_handler]
pub async fn get_user(
    State(postgres_service): State<PostgresService>,
    Extension(uuid): Extension<Uuid>,
) -> Response {
    match postgres_service.find_user_by_uuid(uuid).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[axum::debug_handler]
pub async fn post_user(
    State(postgres_service): State<PostgresService>,
    Json(user): Json<CreateUserRequest>,
) -> Response {
    match postgres_service
        .create_user(user.email, user.password)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[axum::debug_handler]
pub async fn delete_user(
    State(postgres_service): State<PostgresService>,
    Extension(uuid): Extension<Uuid>,
) -> Response {
    match postgres_service.delete_user(uuid).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

#[axum::debug_handler]
pub async fn update_password(
    State(postgres_service): State<PostgresService>,
    Extension(uuid): Extension<Uuid>,
    Json(user): Json<UpdateUserPasswordRequest>,
) -> Response {
    match postgres_service
        .update_user_password(uuid, user.password)
        .await
    {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
