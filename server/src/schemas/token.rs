use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{middleware::auth::REFRESH_TOKEN_COOKIE, schemas::json};

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        let token_type = String::from("Bearer");
        Self {
            access_token,
            refresh_token,
            token_type,
        }
    }
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (
                    header::SET_COOKIE,
                    &refresh_token_cookie(&self.refresh_token),
                ),
            ],
            json(self),
        )
            .into_response()
    }
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

impl RefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        let token_type = String::from("Bearer");
        Self {
            access_token,
            refresh_token,
            token_type,
        }
    }
}

impl IntoResponse for RefreshResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
            ],
            json(self),
        )
            .into_response()
    }
}

fn refresh_token_cookie(refresh_token: &str) -> String {
    format!(
        "{}={}; HttpOnly; Secure; SameSite=Strict",
        REFRESH_TOKEN_COOKIE, refresh_token
    )
}
