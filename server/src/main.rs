mod config;
mod error;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schemas;
mod services;
mod state;

use std::time::Duration;

use axum::{Router, response::Response};
use error::ErrorResponse;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::Level;

use crate::state::AppState;

pub type ApiResult = Result<Response, ErrorResponse>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::init().await?;
    let app = Router::new()
        .merge(routes::api_router(state))
        .fallback_service(routes::web_service())
        .method_not_allowed_fallback(routes::method_not_allowed_fallback)
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
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
