use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::state::AppState;
use crate::{ApiResult, error::ErrorResponse};

mod auth;
mod docs;
mod user;

const API_PATH: &str = "/api";
const AUTH_PATH: &str = "/auth";
const USER_PATH: &str = "/user";

const STATIC_ASSETS_PATH: &str = "/dist";
const INDEX_HTML_PATH: &str = "/dist/index.html";

pub fn api_router(state: AppState) -> Router<AppState> {
    Router::new().nest(
        API_PATH,
        Router::new()
            .nest(AUTH_PATH, auth::router())
            .nest(USER_PATH, user::router(state.clone()))
            .merge(docs::router()),
    )
}

pub fn web_service() -> ServeDir<ServeFile> {
    ServeDir::new(STATIC_ASSETS_PATH).fallback(ServeFile::new(INDEX_HTML_PATH))
}

pub async fn method_not_allowed_fallback() -> ApiResult {
    Err(ErrorResponse::method_not_allowed())
}
