use axum::{extract::Request, response::Response};
use tower::ServiceExt;

use crate::state::AppState;
pub async fn test_request(request: Request) -> Response {
    let state = AppState::mock();
    let app = crate::routes::app(state.clone());
    app.oneshot(request).await.unwrap()
}
