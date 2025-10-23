pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod state;

#[cfg(test)]
pub mod test_helpers;

pub async fn bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().init();
    let state = crate::state::AppState::postgres().await?;
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, extract::Request, http::StatusCode};
    use base64::{Engine, prelude::BASE64_STANDARD};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        models::User, routes, schemas::user::PostUserRequest, state::AppState,
        test_helpers::test_request,
    };

    #[tokio::test]
    async fn valid_login() {
        let uri = "/api/auth/login";
        let valid_email_password = format!("{}:{}", User::mock().email, "password123");
        let valid_auth_header = format!("Basic {}", BASE64_STANDARD.encode(valid_email_password));
        let valid_request = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Authorization", valid_auth_header)
            .body(Body::empty())
            .unwrap();
        let response = test_request(valid_request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn invalid_login() {
        let uri = "/api/auth/login";
        let invalid_email_password = format!("{}:{}", User::mock().email, "password456");
        let invalid_auth_header =
            format!("Basic {}", BASE64_STANDARD.encode(invalid_email_password));
        let invalid_request = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Authorization", invalid_auth_header)
            .body(Body::empty())
            .unwrap();
        let response = test_request(invalid_request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn get_user_by_uuid() {
        let state = AppState::mock();
        let app = routes::app(state.clone());
        let token = state
            .auth_service
            .generate_access_token(User::mock().id)
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

    #[tokio::test]
    async fn create_user() {
        let state = AppState::mock();
        let app = routes::app(state.clone());
        let token = state
            .auth_service
            .generate_access_token(User::mock().id)
            .unwrap();
        let auth_header = format!("Bearer {}", token);
        let body = PostUserRequest {
            email: String::from("some@mail.com"),
            password: String::from("abc123"),
        };
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/user")
                    .method("POST")
                    .header("Authorization", auth_header)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn delete_user() {
        let state = AppState::mock();
        let app = routes::app(state.clone());
        let token = state
            .auth_service
            .generate_access_token(User::mock().id)
            .unwrap();
        let auth_header = format!("Bearer {}", token);
        let uri = format!("/api/user/{}", User::mock().id);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .method("GET")
                    .header("Authorization", auth_header.clone())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .method("DELETE")
                    .header("Authorization", auth_header.clone())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .method("GET")
                    .header("Authorization", auth_header.clone())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
