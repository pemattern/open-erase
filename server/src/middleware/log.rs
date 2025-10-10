use axum::{extract::Request, middleware::Next};

use crate::{ApiResult, error::ServiceError};

#[axum::debug_middleware]
pub async fn log(request: Request, next: Next) -> ApiResult {
    let extensions = request.extensions();
    if let Some(service_error) = extensions.get::<ServiceError>() {
        tracing::error!("{:#?}", service_error);
    }
    Ok(next.run(request).await)
}
