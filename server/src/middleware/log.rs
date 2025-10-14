use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use crate::error::{AppResult, ServiceError};

#[axum::debug_middleware]
pub async fn log(request: Request, next: Next) -> AppResult<impl IntoResponse> {
    let extensions = request.extensions();
    if let Some(service_error) = extensions.get::<Arc<ServiceError>>() {
        tracing::error!("{:#?}", service_error);
    }
    Ok(next.run(request).await)
}
