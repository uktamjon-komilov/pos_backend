use axum::{
    async_trait,
    extract::{FromRequest, Json},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use axum::body::Body;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use validator::Validate;
use std::collections::HashMap;

pub struct ValidatedPayload<T> {
    pub payload: T,
}

#[async_trait]
impl<S, T> FromRequest<S, Body> for ValidatedPayload<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // Extract raw JSON Value
        let Json(value) = Json::<Value>::from_request(req, state).await.map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(json!({"ok": false, "error": "Invalid JSON format"}))).into_response()
        })?;

        // Deserialize into T
        let payload: T = serde_json::from_value(value).map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(json!({"ok": false, "error": "Missing or invalid fields"}))).into_response()
        })?;

        // Run validation
        payload.validate().map_err(validation_error_response)?;

        Ok(ValidatedPayload { payload })
    }
}

pub fn validation_error_response(err: validator::ValidationErrors) -> Response {
    let mut field_errors = HashMap::new();
    for (field, errors) in err.field_errors() {
        if let Some(validation_error) = errors.get(0) {
            let message = validation_error.message.as_deref().unwrap_or("Invalid value");
            field_errors.insert(field.to_string(), message.to_string());
        }
    }

    (StatusCode::BAD_REQUEST, Json(json!({"ok": false,  "field_errors": field_errors }))).into_response()
}

pub async fn validate_and_extract<T>(Json(value): Json<Value>) -> Result<T, Response>
where
    T: DeserializeOwned + Validate,
{
    let payload: T = serde_json::from_value(value).map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(json!({"ok": false, "error":"Invalid or missing fields"}))).into_response()
    })?;

    payload.validate().map_err(validation_error_response)?;

    Ok(payload)
}
