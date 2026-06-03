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
use flagdrive_shared::{
    DownloadRequest, ErrorResponse, FileListRequest, FlagDriveFileVisibility, UploadMetadata,
    UploadResponse,
};
use rand::prelude::*;

pub async fn get_file_list(
    State(api_state): State<FlagDriveAPIState>,
    Path(username): Path<String>,
    payload: Option<Json<FileListRequest>>,
) -> Response {
    let mut viewer: Option<String> = None;
    if let Some(Json(body)) = payload {
        let token = &body.token;
        if !token.is_empty() {
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
                serde_json::to_string(&ErrorResponse {
                    error: "Failed to retrieve user files".to_string(),
                })
                .unwrap(),
            ))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&files).unwrap()))
        .unwrap()
}

pub async fn upload_file(
    State(api_state): State<FlagDriveAPIState>,
    mut multipart: Multipart,
) -> Response {
    let mut token = String::new();
    let mut name = String::new();
    let mut encryption_key = String::new();
    let mut visibility = FlagDriveFileVisibility::Private;
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
                    if let Ok(payload) = serde_json::from_slice::<UploadMetadata>(&bytes) {
                        token = payload.token;
                        encryption_key = payload.encryption_key;
                        visibility = payload.visibility;
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
                serde_json::to_string(&ErrorResponse {
                    error: "Token, name, and file content are required".to_string(),
                })
                .unwrap(),
            ))
            .unwrap();
    }

    let Ok(username) = get_username_from_token(&api_state.pool, &token).await else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ErrorResponse {
                    error: "Invalid token".to_string(),
                })
                .unwrap(),
            ))
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
        visibility.to_int(),
        &encrypted_content,
        &encryption_key,
    )
    .await
    {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ErrorResponse {
                    error: format!("Failed to save file: {}", err),
                })
                .unwrap(),
            ))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&UploadResponse { file_id: id }).unwrap(),
        ))
        .unwrap()
}

pub async fn download_file(
    State(api_state): State<FlagDriveAPIState>,
    Path(file_id): Path<u64>,
    Json(payload): Json<DownloadRequest>,
) -> Response {
    let token = &payload.token;
    let decryption_key = payload.decryption_key.as_deref();

    let Ok((file, content, real_enc_key)) =
        get_download_file(&api_state.pool, file_id as i64).await
    else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ErrorResponse {
                    error: "File not found".to_string(),
                })
                .unwrap(),
            ))
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
                    serde_json::to_string(&ErrorResponse {
                        error: "Token is required".to_string(),
                    })
                    .unwrap(),
                ))
                .unwrap();
        }

        match get_username_from_token(&api_state.pool, token).await {
            Ok(u) => username = u,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&ErrorResponse {
                            error: "Invalid token".to_string(),
                        })
                        .unwrap(),
                    ))
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
            .body(Body::from(
                serde_json::to_string(&ErrorResponse {
                    error: "Access denied".to_string(),
                })
                .unwrap(),
            ))
            .unwrap();
    }

    if let Some(dec_key) = decryption_key {
        if !real_enc_key.is_empty() && dec_key != real_enc_key {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&ErrorResponse {
                        error: "Invalid decryption key".to_string(),
                    })
                    .unwrap(),
                ))
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

