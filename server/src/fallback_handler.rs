use crate::{ApiResult, error::ErrorResponse};

#[axum::debug_handler]
pub async fn not_found_handler() -> ApiResult {
    ErrorResponse::not_found()
}

#[axum::debug_handler]
pub async fn method_not_allowed_handler() -> ApiResult {
    ErrorResponse::method_not_allowed()
}
