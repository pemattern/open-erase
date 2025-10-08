use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

use crate::services::ServiceError;

pub struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl From<ServiceError> for ErrorResponse {
    fn from(error: ServiceError) -> Self {
        tracing::error!("{:?}", error);
        match error {
            ServiceError::Database(error) => match error {
                sqlx::Error::RowNotFound => ErrorResponse::not_found(),
                _ => ErrorResponse::internal_server_error(),
            },
            ServiceError::Hash(error) => match error {
                argon2::password_hash::Error::Password => ErrorResponse::unauthorized(),
                _ => ErrorResponse::internal_server_error(),
            },
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
