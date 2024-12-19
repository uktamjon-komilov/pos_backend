use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AuthLoginPayload {
    #[validate(required(message = "Required field"), email)]
    pub username: Option<String>,

    #[validate(required(message = "Required field"))]
    pub password: Option<String>,

    #[validate(required(message = "Required field"))]
    pub device_name: Option<String>,

    #[validate(required(message = "Required field"))]
    pub terminal_type: Option<String>,
}

#[derive(Serialize)]
pub struct AuthLoginResponse {
    pub terminal_type: String,
    pub terminal_token: String,
}