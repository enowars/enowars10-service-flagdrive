use crate::FlagDriveAPIState;
use crate::database::{
    add_upload_file, get_download_file, get_user_files, get_username_from_token,
};
use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rand::RngExt;
use serde_json::{Value, json};

pub fn xor_cipher(data: &[u8], key: &str) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }
    let key_bytes = key.as_bytes();
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect()
}

pub async fn get_file_list(
    State(api_state): State<FlagDriveAPIState>,
    Path(username): Path<String>,
) -> Response {
    let Ok(files) = get_user_files(&api_state.pool, &username).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to retrieve user files" })),
        )
            .into_response();
    };

    (StatusCode::OK, Json(files)).into_response()
}

pub async fn upload_file(
    State(api_state): State<FlagDriveAPIState>,
    mut multipart: Multipart,
) -> Response {
    let mut token = String::new();
    let mut name = String::new();
    let mut encryption_key = String::new();
    let mut visibility = 0;
    let mut content_bytes = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name_str = field.name().unwrap_or("").to_string();
        match name_str.as_str() {
            "file" => {
                if let Some(fname) = field.file_name() {
                    name = fname.to_string();
                }
                if let Ok(bytes) = field.bytes().await {
                    content_bytes = bytes.to_vec();
                }
            }
            "json" => {
                if let Ok(bytes) = field.bytes().await {
                    if let Ok(payload) = serde_json::from_slice::<Value>(&bytes) {
                        if let Some(t) = payload.get("token").and_then(|v| v.as_str()) {
                            token = t.to_string();
                        }
                        if let Some(k) = payload.get("encryption_key").and_then(|v| v.as_str()) {
                            encryption_key = k.to_string();
                        }
                        if let Some(v) = payload.get("visibility").and_then(|v| v.as_i64()) {
                            visibility = v as i32;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let id = rand::rng().random::<u64>();

    if token.is_empty() || name.is_empty() || content_bytes.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Token, name, and file content are required" })),
        )
            .into_response();
    }

    let Ok(username) = get_username_from_token(&api_state.pool, &token).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response();
    };

    let encrypted_content = xor_cipher(&content_bytes, &encryption_key);

    if let Err(err) = add_upload_file(
        &api_state.pool,
        id as i64,
        &name,
        &username,
        visibility,
        &encrypted_content,
        &encryption_key,
    )
    .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to save file: {}", err) })),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "file_id": id as u64,
        })),
    )
        .into_response()
}

pub async fn download_file(
    State(api_state): State<FlagDriveAPIState>,
    Path(file_id): Path<u64>,
    Json(payload): Json<Value>,
) -> Response {
    let token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");
    let decryption_key = payload.get("decryption_key").and_then(|v| v.as_str());

    if token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Token is required" })),
        )
            .into_response();
    }

    let Ok(username) = get_username_from_token(&api_state.pool, token).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response();
    };

    let Ok(file) = get_download_file(&api_state.pool, file_id as i64).await else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "File not found" })),
        )
            .into_response();
    };

    let mut has_access = file.owner == username;

    if !has_access {
        if let Some(dec_key) = decryption_key {
            if !file.encryption_key.is_empty() && dec_key == file.encryption_key {
                has_access = true;
            }
        }

        if !has_access {
            if let Ok(visible_files) = get_user_files(&api_state.pool, &username).await {
                if visible_files.iter().any(|f| f.id == file.id) {
                    has_access = true;
                }
            }
        }

        if !has_access {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "Access denied" })),
            )
                .into_response();
        }
    }

    let returned_content = if let Some(dec_key) = decryption_key {
        xor_cipher(&file.content, dec_key)
    } else {
        file.content.clone()
    };

    let content_disposition = format!("attachment; filename=\"{}\"", file.name);

    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/octet-stream")
        .header("content-disposition", content_disposition)
        .body(axum::body::Body::from(returned_content))
        .unwrap()
        .into_response()
}
