mod config;
mod domain;
mod error;
mod middleware;
mod repositories;
mod routes;
mod services;
mod utils;

use std::{env, time::Duration};

use axum::{Router, response::Response};
use config::SERVER_CONFIG;
use error::ErrorResponse;
use services::PostgresService;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::Level;

pub type ApiResult = Result<Response, ErrorResponse>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().init();
    tracing::info!("initialized tracing subscriber");
    tracing::info!("loaded server config: {:#?}", *SERVER_CONFIG);

    let db_url = db_url_from_envs()?;
    let pool = PgPoolOptions::new().connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let postgres_service = PostgresService::new(&pool);
    let app = Router::new()
        .merge(routes::api_router(postgres_service))
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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn db_url_from_envs() -> Result<String, Box<dyn std::error::Error>> {
    let username = env::var("POSTGRES_USER")?;
    let password = env::var("POSTGRES_PASSWORD")?;
    let host = env::var("POSTGRES_HOST")?;
    let port = env::var("POSTGRES_PORT")?;
    let db = env::var("POSTGRES_DB")?;
    let url = format!("postgres://{username}:{password}@{host}:{port}/{db}");
    Ok(url)
}
