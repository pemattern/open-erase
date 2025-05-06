use axum::{Router, handler::Handler};
use tower_http::services::{ServeDir, ServeFile};

use crate::{ApiResult, error::ErrorResponse};

mod auth;
mod docs;
mod user;

const API_PATH: &str = "/api";
const AUTH_PATH: &str = "/auth";
const USER_PATH: &str = "/user";

pub fn api_router() -> Router {
    Router::new().nest(
        API_PATH,
        Router::new()
            .nest(AUTH_PATH, auth::router())
            .nest(USER_PATH, user::router())
            .merge(docs::router()),
    )
}

pub fn web_service() -> ServeDir<ServeFile> {
    ServeDir::new("dist/").fallback(ServeFile::new("dist/index.html"))
}

pub async fn method_not_allowed_fallback() -> ApiResult {
    ErrorResponse::method_not_allowed()
}
