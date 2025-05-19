mod config;
mod error;
mod routes;

use std::{env, time::Duration};

use axum::{Extension, Router, response::Response};
use error::ErrorResponse;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::Level;

pub type ApiResult = Result<Response, ErrorResponse>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().init();
    tracing::info!("initialized tracing subscriber");

    tracing::info!("reading environment variables");
    let db_url = db_url_from_envs()?;
    tracing::info!("connecting to database");
    let pool = PgPoolOptions::new().connect(&db_url).await?;

    tracing::info!("running migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!("creating router");
    let app = Router::new()
        .merge(routes::api_router())
        .fallback_service(routes::web_service())
        .method_not_allowed_fallback(routes::method_not_allowed_fallback)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn db_url_from_envs() -> Result<String, Box<dyn std::error::Error>> {
    let username = env::var("POSTGRES_USER").unwrap_or("postgres".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
    let db = env::var("POSTGRES_DB").unwrap_or("postgres".to_string());
    let url = format!("postgres://{username}:{password}@{host}:{port}/{db}");
    Ok(url)
}
