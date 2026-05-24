use crate::FlagDriveAPIState;
use crate::database::{delete_token, get_username_from_token};
use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
use serde_json::{Value, json};

pub async fn verify_token(
    State(state): State<FlagDriveAPIState>,
    Json(payload): Json<Value>,
) -> Response {
    let token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");

    match get_username_from_token(&state.pool, token).await {
        Ok(username) => Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "username": username }).to_string()))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
            .unwrap(),
    }
}

pub async fn logout_token(
    State(state): State<FlagDriveAPIState>,
    Json(payload): Json<Value>,
) -> Response {
    let token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");

    if get_username_from_token(&state.pool, token).await.is_err() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
            .unwrap();
    }

    if delete_token(&state.pool, token).await.is_err() {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Failed to delete token" }).to_string(),
            ))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "message": "Logged out successfully" }).to_string(),
        ))
        .unwrap()
}
