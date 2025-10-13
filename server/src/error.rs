use std::sync::Arc;

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

pub enum AppError {
    Client(ClientError),
    Server(ServerError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Client(client_error) => client_error.into_response(),
            AppError::Server(server_error) => server_error.into_response(),
        }
    }
}

impl From<ClientError> for AppError {
    fn from(value: ClientError) -> Self {
        Self::Client(value)
    }
}

impl From<ServerError> for AppError {
    fn from(value: ServerError) -> Self {
        Self::Server(value)
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
pub enum ServerError {
    Database(DatabaseError),
    Hash(argon2::password_hash::Error),
    Token(jsonwebtoken::errors::Error),
    Uuid(uuid::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let mut error_response = ErrorResponse::internal_server_error().into_response();
        error_response.extensions_mut().insert(Arc::new(self));
        error_response
    }
}

impl From<DatabaseError> for ServerError {
    fn from(value: DatabaseError) -> Self {
        Self::Database(value)
    }
}

impl From<argon2::password_hash::Error> for ServerError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Hash(value)
    }
}

impl From<jsonwebtoken::errors::Error> for ServerError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Token(value)
    }
}

impl From<uuid::Error> for ServerError {
    fn from(value: uuid::Error) -> Self {
        Self::Uuid(value)
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    Postgres(sqlx::Error),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        Self::Postgres(value)
    }
}

struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl From<ServerError> for ErrorResponse {
    fn from(error: ServerError) -> Self {
        tracing::error!("{:?}", error);
        match error {
            ServerError::Database(_error) => ErrorResponse::internal_server_error(),
            ServerError::Hash(_error) => ErrorResponse::unauthorized(),
            ServerError::Token(_error) => ErrorResponse::internal_server_error(),
            ServerError::Uuid(_error) => ErrorResponse::internal_server_error(),
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
