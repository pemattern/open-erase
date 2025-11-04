use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use open_erase_lib::schemas::user::{
    DeleteUserResponse, GetUserResponse, PatchUserRequest, PatchUserResponse, PostUserRequest,
    PostUserResponse,
};
use serde::{Deserialize, Serialize};

use crate::{models::User, schemas::json};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerGetUserResponse(pub GetUserResponse);

impl IntoResponse for ServerGetUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.0)).into_response()
    }
}

impl From<User> for ServerGetUserResponse {
    fn from(value: User) -> Self {
        Self(GetUserResponse {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerPostUserRequest(pub PostUserRequest);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerPostUserResponse(pub PostUserResponse);

impl IntoResponse for ServerPostUserResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/{}", self.0.id))],
            json(self.0),
        )
            .into_response()
    }
}

impl From<User> for ServerPostUserResponse {
    fn from(value: User) -> Self {
        Self(PostUserResponse {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerPatchUserRequest(pub PatchUserRequest);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerPatchUserResponse(pub PatchUserResponse);

impl IntoResponse for ServerPatchUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.0)).into_response()
    }
}

impl From<User> for ServerPatchUserResponse {
    fn from(value: User) -> Self {
        Self(PatchUserResponse {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerDeleteUserResponse(pub DeleteUserResponse);

impl IntoResponse for ServerDeleteUserResponse {
    fn into_response(self) -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

impl From<User> for ServerDeleteUserResponse {
    fn from(value: User) -> Self {
        Self(DeleteUserResponse { id: value.id })
    }
}
