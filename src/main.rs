mod migrations;

use std::{env, time::Duration};

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    let db_url = db_url_from_envs();
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

    let app = Router::new().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(TimeoutLayer::new(Duration::from_secs(5))),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    tracing::info!("initialized tracing subscriber");
}

pub fn db_url_from_envs() -> String {
    let username = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let host = env::var("POSTGRES_HOST").unwrap();
    let port = env::var("POSTGRES_PORT").unwrap();
    let db = env::var("POSTGRES_DB").unwrap();
    let url = format!("postgres://{username}:{password}@{host}:{port}/{db}");
    tracing::info!("db url: {url}");
    url
}
