use crate::FlagDriveAPIState;
use crate::crypto::{aes_gcm_decrypt, aes_gcm_encrypt};
use crate::database::{
    add_upload_file, get_download_file, get_user_encryption_key, get_user_files,
    get_username_from_token,
};
use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::Response,
};
use flagdrive_shared::FlagDriveFileVisibility;
use rand::prelude::*;
use serde_json::{Value, json};

pub async fn get_file_list(
    State(api_state): State<FlagDriveAPIState>,
    Path(username): Path<String>,
    payload: Option<Json<Value>>,
) -> Response {
    let mut viewer: Option<String> = None;
    if let Some(Json(body)) = payload {
        if let Some(token) = body.get("token").and_then(|v| v.as_str()) {
            if let Ok(v) = get_username_from_token(&api_state.pool, token).await {
                viewer = Some(v);
            }
        }
    }

    let Ok(files) = get_user_files(&api_state.pool, &username, viewer.as_deref()).await else {
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

    let user_key = get_user_encryption_key(&api_state.pool, &username)
        .await
        .unwrap_or_default();
    let encrypted_content = aes_gcm_encrypt(&content_bytes, &user_key, &encryption_key);

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

    let Ok((file, content, real_enc_key)) =
        get_download_file(&api_state.pool, file_id as i64).await
    else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "File not found" }).to_string()))
            .unwrap();
    };

    let is_public = file.visibility == FlagDriveFileVisibility::Public;
    let mut username = String::new();

    if !is_public {
        if token.is_empty() {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "error": "Token is required" }).to_string(),
                ))
                .unwrap();
        }

        match get_username_from_token(&api_state.pool, token).await {
            Ok(u) => username = u,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
                    .unwrap();
            }
        }
    }

    let has_access = is_public
        || file.owner == username
        || get_user_files(&api_state.pool, &username, Some(&username))
            .await
            .is_ok_and(|files| files.iter().any(|f| f.id == file.id));

    if !has_access {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Access denied" }).to_string()))
            .unwrap();
    }

    if let Some(dec_key) = decryption_key {
        if !real_enc_key.is_empty() && dec_key != real_enc_key {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("content-type", "application/json")
                .body(Body::from(json!({ "error": "Invalid decryption key" }).to_string()))
                .unwrap();
        }
    }

    let returned_content = if let Some(dec_key) = decryption_key {
        let owner_key = get_user_encryption_key(&api_state.pool, &file.owner)
            .await
            .unwrap_or_default();
        aes_gcm_decrypt(&content, &owner_key, dec_key)
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
