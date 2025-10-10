use std::sync::Arc;

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

#[derive(Debug, Clone)]
pub enum ServiceError {
    Database(DatabaseError),
    Hash(argon2::password_hash::Error),
    Token(jsonwebtoken::errors::Error),
    Uuid(uuid::Error),
}

impl From<DatabaseError> for ServiceError {
    fn from(value: DatabaseError) -> Self {
        Self::Database(value)
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

#[derive(Debug, Clone)]
pub enum DatabaseError {
    // Arc required because sqlx::Error doesnt derive Clone
    Postgres(Arc<sqlx::Error>),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        Self::Postgres(Arc::new(value))
    }
}

pub struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let mut error_response = match self {
            ServiceError::Hash(_) => ErrorResponse::unauthorized(),
            _ => ErrorResponse::internal_server_error(),
        }
        .into_response();
        error_response.extensions_mut().insert(self);
        error_response
    }
}

impl From<ServiceError> for ErrorResponse {
    fn from(error: ServiceError) -> Self {
        tracing::error!("{:?}", error);
        match error {
            ServiceError::Database(_error) => ErrorResponse::internal_server_error(),
            ServiceError::Hash(_error) => ErrorResponse::unauthorized(),
            ServiceError::Token(_error) => ErrorResponse::internal_server_error(),
            ServiceError::Uuid(_error) => ErrorResponse::internal_server_error(),
        }
    }
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
