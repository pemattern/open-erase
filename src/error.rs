use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

use crate::ApiResult;

pub struct ErrorResponse {
    status_code: u16,
    message: String,
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
