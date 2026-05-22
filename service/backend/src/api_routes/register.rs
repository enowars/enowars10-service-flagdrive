use crate::FlagDriveAPIState;
use crate::database::{create_new_token, create_new_user};
use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
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
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Username and password are required" }).to_string(),
            ))
            .unwrap();
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
        return Response::builder()
            .status(StatusCode::CONFLICT)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "User already exists" }).to_string(),
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
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "token": token, "username": username }).to_string(),
        ))
        .unwrap()
}
