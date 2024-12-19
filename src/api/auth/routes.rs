use uuid::Uuid;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::api::auth::schemas::{AuthLoginPayload, AuthLoginResponse};
use crate::utils::payload::ValidatedPayload;
use crate::utils::response::ApiResponse;

pub async fn auth_login_handler(
    ValidatedPayload { payload }: ValidatedPayload<AuthLoginPayload>,
) -> impl IntoResponse {
    let resp = AuthLoginResponse {
        terminal_type: payload.terminal_type.unwrap(),
        terminal_token: Uuid::new_v4().to_string(),
    };

    // Wrap the response using ApiResponse::success
    (StatusCode::OK, ApiResponse::success(resp))
}