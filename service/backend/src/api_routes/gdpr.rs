use crate::{
    FlagDriveAPIState,
    database::{
        get_gdpr_data, get_user_by_username, get_user_files, get_username_from_token,
        insert_gdpr_data,
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rand::prelude::*;
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn gdpr_request_user_data(
    State(api_state): State<FlagDriveAPIState>,
    Json(payload): Json<Value>,
) -> Response {
    let token = match payload.get("token").and_then(|v| v.as_str()) {
        Some(t) if !t.is_empty() => t,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Token is required" })),
            )
                .into_response();
        }
    };

    let username = match get_username_from_token(&api_state.pool, token).await {
        Ok(name) => name,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
                .into_response();
        }
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    let nonce: String =
        std::iter::repeat_with(|| *b"0123456789abcdef".choose(&mut rand::rng()).unwrap() as char)
            .take(32)
            .collect();

    let filenames: Vec<String> = get_user_files(&api_state.pool, &username)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|f| f.name)
        .collect();

    let content_str = json!({
        "username": &username,
        "exported_at": timestamp,
        "files": filenames
    })
    .to_string();

    let gdpr_id = format!("{}-{}-{}", &username, timestamp, &nonce);

    if insert_gdpr_data(
        &api_state.pool,
        &username,
        timestamp as i64,
        &nonce,
        &content_str,
    )
    .await
    .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to store GDPR request in database" })),
        )
            .into_response();
    }

    (StatusCode::OK, Json(json!({ "gdpr_id": gdpr_id }))).into_response()
}

pub async fn gdpr_download_user_data(
    State(api_state): State<FlagDriveAPIState>,
    Path(gdpr_id): Path<String>,
) -> Response {
    let parts: Vec<&str> = gdpr_id.split('-').collect();
    if parts.len() < 3 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid GDPR ID format" })),
        )
            .into_response();
    }

    let username = parts[0];
    let timestamp_or_latest = parts[1];
    let nonce = parts[2];

    if get_user_by_username(&api_state.pool, username, None)
        .await
        .is_err()
    {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        )
            .into_response();
    }

    let chars: Vec<char> = nonce.chars().collect();
    if chars.is_empty()
        || chars
            .iter()
            .take(chars.len().saturating_sub(1).max(1))
            .any(|c| !c.is_ascii_hexdigit())
    {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Invalid GDPR ID format" })),
        )
            .into_response();
    }

    let Ok(content_str) =
        get_gdpr_data(&api_state.pool, username, timestamp_or_latest, nonce).await
    else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "GDPR export request not found or expired" })),
        )
            .into_response();
    };

    let headers = [
        ("content-type", "application/json"),
        (
            "content-disposition",
            "attachment; filename=\"gdpr_export.json\"",
        ),
    ];

    (StatusCode::OK, headers, content_str).into_response()
}
