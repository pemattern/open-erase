use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

use crate::{ApiResult, services::ServiceError};

pub struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl From<ServiceError> for ErrorResponse {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::Database(error) => tracing::error!("{}", error),
            ServiceError::Hash(error) => tracing::error!("{}", error),
            ServiceError::Auth => return ErrorResponse::unauthorized().unwrap_err(),
        };
        ErrorResponse::internal_server_error().unwrap_err()
    }
}

impl ErrorResponse {
    pub fn unauthorized() -> ApiResult {
        Err(Self {
            status_code: 401,
            message: String::from("unauthorized access requested"),
        })
    }

    pub fn not_found() -> ApiResult {
        Err(Self {
            status_code: 404,
            message: String::from("the requested resource was not found"),
        })
    }

    pub fn method_not_allowed() -> ApiResult {
        Err(Self {
            status_code: 405,
            message: String::from("used http method is not allowed for the requested resource"),
        })
    }

    pub fn internal_server_error() -> ApiResult {
        Err(Self {
            status_code: 500,
            message: String::from("an unexpected error occured"),
        })
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
