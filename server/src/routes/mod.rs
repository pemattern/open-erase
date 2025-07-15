use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::services::PostgresService;
use crate::{ApiResult, error::ErrorResponse};

mod auth;
mod docs;
mod user;

const API_PATH: &str = "/api";
const AUTH_PATH: &str = "/auth";
const USER_PATH: &str = "/user";

const STATIC_ASSETS_PATH: &str = "/dist";
const INDEX_HTML_PATH: &str = "/dist/index.html";

pub fn api_router(postgres_service: PostgresService) -> Router {
    Router::new().nest(
        API_PATH,
        Router::new()
            .nest(AUTH_PATH, auth::router(postgres_service.clone()))
            .nest(USER_PATH, user::router(postgres_service.clone()))
            .merge(docs::router()),
    )
}

pub fn web_service() -> ServeDir<ServeFile> {
    ServeDir::new(STATIC_ASSETS_PATH).fallback(ServeFile::new(INDEX_HTML_PATH))
}

pub async fn method_not_allowed_fallback() -> ApiResult {
    ErrorResponse::method_not_allowed()
}
