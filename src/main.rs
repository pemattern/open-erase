mod migrations;

use std::time::Duration;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

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
}
