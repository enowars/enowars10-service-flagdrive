use crate::FlagDriveAPIState;
use crate::database::{check_user_password, create_new_token};
use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
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
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Username and password are required" }).to_string(),
            ))
            .unwrap();
    }

    if !check_user_password(&api_state.pool, username, password)
        .await
        .unwrap_or(false)
    {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Invalid credentials" }).to_string(),
            ))
            .unwrap();
    }

    let Ok(token) = create_new_token(&api_state.pool, username).await else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Failed to generate secure session token" }).to_string(),
            ))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "token": token, "username": username }).to_string(),
        ))
        .unwrap()
}
