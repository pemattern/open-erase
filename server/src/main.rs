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

use crate::state::AppState;

pub type ApiResult = Result<Response, ErrorResponse>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::postgres().await?;
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{models::User, services::token::TokenService};

    use super::*;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use base64::{Engine, prelude::BASE64_STANDARD};
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn login() {
        let state = AppState::mock();
        let app = routes::app(state.clone());
        let uri = "/api/auth/login";
        let valid_email_password = format!("{}:{}", User::mock().email, "password123");
        let valid_auth_header = format!("Basic {}", BASE64_STANDARD.encode(valid_email_password));

        let invalid_email_password = format!("{}:{}", User::mock().email, "password456");
        let invalid_auth_header =
            format!("Basic {}", BASE64_STANDARD.encode(invalid_email_password));

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("Authorization", valid_auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("Authorization", invalid_auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn get_user_by_uuid() {
        let state = AppState::mock();
        let app = routes::app(state.clone());
        let token = TokenService
            .generate_access_token(Uuid::default(), &state.config)
            .unwrap();
        let uri = format!("/api/user/{}", Uuid::default());
        let auth_header = format!("Bearer {}", token);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .header("Authorization", auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
