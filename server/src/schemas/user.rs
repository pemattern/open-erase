use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::User;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl IntoResponse for GetUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<User> for GetUserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct PostUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl IntoResponse for PostUserResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/{}", self.id))],
            Json(self),
        )
            .into_response()
    }
}

impl From<User> for PostUserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct DeleteUserResponse {
    pub id: Uuid,
}

impl IntoResponse for DeleteUserResponse {
    fn into_response(self) -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

impl From<User> for DeleteUserResponse {
    fn from(value: User) -> Self {
        Self { id: value.id }
    }
}
