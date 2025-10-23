use axum::{body::Body, extract::Request, http::StatusCode};
use base64::{Engine, prelude::BASE64_STANDARD};
use server::{models::User, schemas::user::PostUserRequest, state::AppState};
use tower::ServiceExt;
use uuid::Uuid;
