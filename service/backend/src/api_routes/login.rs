use crate::FlagDriveAPIState;
use crate::database::{check_user_password, create_new_token};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

pub async fn login_as_user(
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

    if check_user_password(&api_state.pool, username, password)
        .await
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid credentials" })),
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
        StatusCode::OK,
        Json(json!({ "token": token, "username": username })),
    )
        .into_response()
}
