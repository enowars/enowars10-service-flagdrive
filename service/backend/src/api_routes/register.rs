use crate::FlagDriveAPIState;
use crate::database::{create_new_token, create_new_user};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rand::prelude::*;
use serde_json::{Value, json};

pub async fn register_new_user(
    State(api_state): State<FlagDriveAPIState>,
    Json(payload): Json<Value>,
) -> Response {
    let username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let password = payload
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if username.is_empty() || password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Username and password are required" })),
        )
            .into_response();
    }

    let encryption_key = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(256)
        .map(char::from)
        .collect::<String>();

    if create_new_user(&api_state.pool, username, password, &encryption_key)
        .await
        .is_err()
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "error": "User already exists" })),
        )
            .into_response();
    }

    let Ok(token) = create_new_token(&api_state.pool, username).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to generate secure session token" })),
        )
            .into_response();
    };

    (
        StatusCode::CREATED,
        Json(json!({ "token": token, "username": username })),
    )
        .into_response()
}
