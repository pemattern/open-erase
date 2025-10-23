use std::sync::Arc;

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;
pub type ServiceResult<T> = Result<T, ServiceError>;
pub type RepositoryResult<T> = Result<T, RepositoryError>;

pub enum AppError {
    Client(ClientError),
    Service(ServiceError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Client(client_error) => client_error.into_response(),
            AppError::Service(service_error) => service_error.into_response(),
        }
    }
}

impl From<ClientError> for AppError {
    fn from(value: ClientError) -> Self {
        Self::Client(value)
    }
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        Self::Service(value)
    }
}

pub enum ClientError {
    MethodNotAllowed,
    NotFound,
    Unauthorized,
}

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        let error_response = match &self {
            ClientError::MethodNotAllowed => ErrorResponse::method_not_allowed(),
            ClientError::NotFound => ErrorResponse::not_found(),
            ClientError::Unauthorized => ErrorResponse::unauthorized(),
        };
        error_response.into_response()
    }
}

#[derive(Debug)]
pub enum ServiceError {
    Repository(RepositoryError),
    Hash(argon2::password_hash::Error),
    Token(jsonwebtoken::errors::Error),
    Parsing,
    Validation,
    Uuid(uuid::Error),
    Serialization(serde_json::Error),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let mut error_response = ErrorResponse::internal_server_error().into_response();
        error_response.extensions_mut().insert(Arc::new(self));
        error_response
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(value: RepositoryError) -> Self {
        Self::Repository(value)
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Hash(value)
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Token(value)
    }
}

impl From<uuid::Error> for ServiceError {
    fn from(value: uuid::Error) -> Self {
        Self::Uuid(value)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RepositoryError {
    Sqlx(sqlx::Error),
    Test,
}

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl ErrorResponse {
    pub fn unauthorized() -> Self {
        Self {
            status_code: 401,
            message: String::from("unauthorized access requested"),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            message: String::from("the requested resource was not found"),
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            status_code: 405,
            message: String::from("used http method is not allowed for the requested resource"),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status_code: 500,
            message: String::from("an unexpected error occured"),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status_code)
            .body(Body::new(self.message.to_string()))
            .unwrap()
    }
}
