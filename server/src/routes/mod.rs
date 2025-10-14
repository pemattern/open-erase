use std::time::Duration;

use axum::{Router, middleware};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Level;

use crate::error::{AppResult, ClientError};
use crate::middleware::{
    auth::{validate_basic_auth, validate_jwt},
    log::log,
};
use crate::state::AppState;

mod auth;
mod docs;
mod user;

const API_PATH: &str = "/api";
const AUTH_PATH: &str = "/auth";
const USER_PATH: &str = "/user";

const STATIC_ASSETS_PATH: &str = "/dist";
const INDEX_HTML_PATH: &str = "/dist/index.html";

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(api_router(state.clone()))
        .fallback_service(web_service())
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            tower_http::trace::DefaultMakeSpan::new().level(Level::INFO),
                        )
                        .on_response(
                            tower_http::trace::DefaultOnResponse::new().level(Level::INFO),
                        ),
                )
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .layer(middleware::from_fn(log)),
        )
        .with_state(state)
}

pub fn api_router(state: AppState) -> Router<AppState> {
    Router::new().nest(
        API_PATH,
        Router::new()
            .merge(basic_auth_router(state.clone()))
            .merge(jwt_auth_router(state.clone()))
            .merge(docs::router()),
    )
}

pub fn basic_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest(AUTH_PATH, auth::router())
        .layer(middleware::from_fn_with_state(state, validate_basic_auth))
}

pub fn jwt_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest(USER_PATH, user::router())
        .layer(middleware::from_fn_with_state(state, validate_jwt))
}

pub fn web_service() -> ServeDir<ServeFile> {
    ServeDir::new(STATIC_ASSETS_PATH).fallback(ServeFile::new(INDEX_HTML_PATH))
}

pub async fn method_not_allowed_fallback() -> AppResult<()> {
    Err(ClientError::MethodNotAllowed.into())
}
