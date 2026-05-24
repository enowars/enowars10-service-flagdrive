use crate::FlagDriveAPIState;
use crate::database::{
    add_upload_file, get_download_file, get_user_files, get_username_from_token,
};
use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::Response,
};
use rand::prelude::*;
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
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Failed to retrieve user files" }).to_string(),
            ))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!(files).to_string()))
        .unwrap()
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
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Token, name, and file content are required" }).to_string(),
            ))
            .unwrap();
    }

    let Ok(username) = get_username_from_token(&api_state.pool, &token).await else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
            .unwrap();
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
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": format!("Failed to save file: {}", err) }).to_string(),
            ))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "file_id": id as u64,
            })
            .to_string(),
        ))
        .unwrap()
}

pub async fn download_file(
    State(api_state): State<FlagDriveAPIState>,
    Path(file_id): Path<u64>,
    Json(payload): Json<Value>,
) -> Response {
    let token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");
    let decryption_key = payload.get("decryption_key").and_then(|v| v.as_str());

    if token.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Token is required" }).to_string(),
            ))
            .unwrap();
    }

    let Ok(username) = get_username_from_token(&api_state.pool, token).await else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
            .unwrap();
    };

    let Ok((file, content, real_enc_key)) =
        get_download_file(&api_state.pool, file_id as i64).await
    else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "File not found" }).to_string()))
            .unwrap();
    };

    let has_access = file.owner == username
        || decryption_key.is_some_and(|key| !real_enc_key.is_empty() && key == real_enc_key)
        || get_user_files(&api_state.pool, &username)
            .await
            .is_ok_and(|files| files.iter().any(|f| f.id == file.id));

    if !has_access {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Access denied" }).to_string()))
            .unwrap();
    }

    let returned_content = if let Some(dec_key) = decryption_key {
        xor_cipher(&content, dec_key)
    } else {
        content.clone()
    };

    let content_disposition = format!("attachment; filename=\"{}\"", file.name);

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/octet-stream")
        .header("content-disposition", content_disposition)
        .body(Body::from(returned_content))
        .unwrap()
}
