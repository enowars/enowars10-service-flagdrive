use crate::FlagDriveAPIState;
use crate::database::{
    follow_user, get_user_by_username, get_username_from_token, is_following, unfollow_user,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

pub async fn get_user_info(
    State(api_state): State<FlagDriveAPIState>,
    Path(viewer_username): Path<String>,
    payload: Option<Json<Value>>,
) -> Response {
    let viewed_username = payload
        .as_ref()
        .and_then(|Json(p)| p.get("username"))
        .and_then(|v| v.as_str())
        .unwrap_or(&viewer_username);

    let Ok(user) =
        get_user_by_username(&api_state.pool, viewed_username, Some(&viewer_username)).await
    else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        )
            .into_response();
    };

    (StatusCode::OK, Json(json!(user))).into_response()
}

pub async fn follow_user_action(
    State(api_state): State<FlagDriveAPIState>,
    Path(target_username): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    let token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");
    let followee = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if token.is_empty() || followee.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Token and username are required" })),
        )
            .into_response();
    }

    let Ok(username_from_token) = get_username_from_token(&api_state.pool, token).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response();
    };

    if username_from_token != target_username {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Unauthorized action" })),
        )
            .into_response();
    }

    if get_user_by_username(&api_state.pool, followee, None)
        .await
        .is_err()
    {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User to follow not found" })),
        )
            .into_response();
    }

    if target_username == followee {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "You cannot follow yourself" })),
        )
            .into_response();
    }

    match is_following(&api_state.pool, &target_username, followee).await {
        Ok(true) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("You already follow {}", followee) })),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Database error checking relationship" })),
            )
                .into_response();
        }
        Ok(false) => {}
    }

    if follow_user(&api_state.pool, &target_username, followee)
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to follow user" })),
        )
            .into_response();
    }

    (StatusCode::OK, Json(json!({ "status": "success", "message": format!("You are now following {}", followee) }))).into_response()
}

pub async fn unfollow_user_action(
    State(api_state): State<FlagDriveAPIState>,
    Path(target_username): Path<String>,
    Json(payload): Json<Value>,
) -> Response {
    let token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");
    let followee = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if token.is_empty() || followee.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Token and username are required" })),
        )
            .into_response();
    }

    let Ok(username_from_token) = get_username_from_token(&api_state.pool, token).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response();
    };

    if username_from_token != target_username {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Unauthorized action" })),
        )
            .into_response();
    }

    match is_following(&api_state.pool, &target_username, followee).await {
        Ok(false) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("You do not follow {}", followee) })),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Database error checking relationship" })),
            )
                .into_response();
        }
        Ok(true) => {}
    }

    if unfollow_user(&api_state.pool, &target_username, followee)
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to unfollow user" })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(
            json!({ "status": "success", "message": format!("You have unfollowed {}", followee) }),
        ),
    )
        .into_response()
}
