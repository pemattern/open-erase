mod config;
mod error;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schemas;
mod services;
mod state;

use axum::response::Response;
use error::ErrorResponse;

use crate::{routes::app, state::AppState};

pub type ApiResult = Result<Response, ErrorResponse>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::postgres().await?;
    let app = app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test() {
        let app = app(AppState::mock());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/user/00000000-0000-0000-000000000000")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
