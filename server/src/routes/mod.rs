use std::time::Duration;

use axum::{Router, middleware, routing::post};
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Level;

use crate::{
    error::{AppResult, ClientError},
    handlers::auth::refresh,
    middleware::auth::validate_refresh_token,
};
use crate::{
    handlers::auth::login,
    middleware::{
        auth::{validate_access_token, validate_basic_auth},
        log::log,
    },
};
use crate::{handlers::auth::logout, state::AppState};

mod docs;
mod user;

const API_PATH: &str = "/api";
const AUTH_PATH: &str = "/auth";
const LOGIN_PATH: &str = "/login";
const LOGOUT_PATH: &str = "/logout";
const REFRESH_PATH: &str = "/refresh";
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
                // so far from testing brotli + default quality is best performer
                .layer(
                    CompressionLayer::new()
                        .quality(CompressionLevel::Default)
                        .br(true)
                        .no_gzip()
                        .no_deflate()
                        .no_zstd(),
                )
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .layer(middleware::from_fn(log)),
        )
        .with_state(state)
}

pub fn api_router(state: AppState) -> Router<AppState> {
    Router::new().nest(
        API_PATH,
        Router::new()
            .nest(
                AUTH_PATH,
                Router::new()
                    .merge(basic_auth_router(state.clone()))
                    .merge(refresh_token_auth_router(state.clone())),
            )
            .merge(access_token_auth_router(state.clone()))
            .merge(docs::router()),
    )
}

fn basic_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(LOGIN_PATH, post(login))
        .layer(middleware::from_fn_with_state(state, validate_basic_auth))
}

fn refresh_token_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(REFRESH_PATH, post(refresh))
        .route(LOGOUT_PATH, post(logout))
        .layer(middleware::from_fn_with_state(
            state,
            validate_refresh_token,
        ))
}

fn access_token_auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest(USER_PATH, user::router())
        .layer(middleware::from_fn_with_state(state, validate_access_token))
}

fn web_service() -> ServeDir<ServeFile> {
    ServeDir::new(STATIC_ASSETS_PATH).fallback(ServeFile::new(INDEX_HTML_PATH))
}

async fn method_not_allowed_fallback() -> AppResult<()> {
    Err(ClientError::MethodNotAllowed.into())
}
