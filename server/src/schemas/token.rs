use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use open_erase_lib::schemas::token::{LoginResponse, RefreshResponse};
use serde::Serialize;

use crate::{middleware::auth::REFRESH_TOKEN_COOKIE, schemas::json};

#[derive(Serialize)]
#[serde(transparent)]
pub struct ServerLoginResponse(pub LoginResponse);

impl ServerLoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self(LoginResponse::new(access_token, refresh_token))
    }
}

impl IntoResponse for ServerLoginResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (
                    header::SET_COOKIE,
                    &set_refresh_token_cookie(&self.0.refresh_token),
                ),
            ],
            json(self),
        )
            .into_response()
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct ServerRefreshResponse(pub RefreshResponse);

impl ServerRefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self(RefreshResponse::new(access_token, refresh_token))
    }
}

impl IntoResponse for ServerRefreshResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (
                    header::SET_COOKIE,
                    &set_refresh_token_cookie(&self.0.refresh_token),
                ),
            ],
            json(self),
        )
            .into_response()
    }
}

fn set_refresh_token_cookie(refresh_token: &str) -> String {
    format!(
        "{}={}; HttpOnly; Secure; SameSite=Strict",
        REFRESH_TOKEN_COOKIE, refresh_token
    )
}

fn reset_refresh_token_cookie() -> String {
    let mut cookie = set_refresh_token_cookie("");
    cookie.push_str("; MaxAge=0");
    cookie
}

#[derive(Serialize)]
pub struct ServerLogoutResponse;

impl IntoResponse for ServerLogoutResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [
                (header::CACHE_CONTROL, "no-store"),
                (header::PRAGMA, "no-cache"),
                (header::SET_COOKIE, &reset_refresh_token_cookie()),
            ],
            json(self),
        )
            .into_response()
    }
}
