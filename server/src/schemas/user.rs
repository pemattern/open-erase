use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::User, schemas::json};

#[derive(Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for GetUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self)).into_response()
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

#[derive(Serialize, Deserialize)]
pub struct PostUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for PostUserResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/{}", self.id))],
            json(self),
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

#[derive(Serialize, Deserialize)]
pub struct PatchUserRequest {
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PatchUserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for PatchUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self)).into_response()
    }
}

impl From<User> for PatchUserResponse {
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
