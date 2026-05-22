use crate::FlagDriveAPIState;
use crate::database::{
    follow_user, get_followers_list, get_following_list, get_user_by_username,
    get_username_from_token, is_following, unfollow_user,
};
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
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
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "User not found" }).to_string()))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!(user).to_string()))
        .unwrap()
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
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Token and username are required" }).to_string(),
            ))
            .unwrap();
    }

    let Ok(username_from_token) = get_username_from_token(&api_state.pool, token).await else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
            .unwrap();
    };

    if username_from_token != target_username {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Unauthorized action" }).to_string(),
            ))
            .unwrap();
    }

    if get_user_by_username(&api_state.pool, followee, None)
        .await
        .is_err()
    {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "User to follow not found" }).to_string(),
            ))
            .unwrap();
    }

    if target_username == followee {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "You cannot follow yourself" }).to_string(),
            ))
            .unwrap();
    }

    match is_following(&api_state.pool, &target_username, followee).await {
        Ok(true) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "error": format!("You already follow {}", followee) }).to_string(),
                ))
                .unwrap();
        }
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "error": "Database error checking relationship" }).to_string(),
                ))
                .unwrap();
        }
        Ok(false) => {}
    }

    if follow_user(&api_state.pool, &target_username, followee)
        .await
        .is_err()
    {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Failed to follow user" }).to_string(),
            ))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "status": "success", "message": format!("You are now following {}", followee) }).to_string()))
        .unwrap()
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
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Token and username are required" }).to_string(),
            ))
            .unwrap();
    }

    let Ok(username_from_token) = get_username_from_token(&api_state.pool, token).await else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(json!({ "error": "Invalid token" }).to_string()))
            .unwrap();
    };

    if username_from_token != target_username {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Unauthorized action" }).to_string(),
            ))
            .unwrap();
    }

    match is_following(&api_state.pool, &target_username, followee).await {
        Ok(false) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "error": format!("You do not follow {}", followee) }).to_string(),
                ))
                .unwrap();
        }
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "error": "Database error checking relationship" }).to_string(),
                ))
                .unwrap();
        }
        Ok(true) => {}
    }

    if unfollow_user(&api_state.pool, &target_username, followee)
        .await
        .is_err()
    {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Failed to unfollow user" }).to_string(),
            ))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "status": "success", "message": format!("You have unfollowed {}", followee) })
                .to_string(),
        ))
        .unwrap()
}

pub async fn get_followers_action(
    State(api_state): State<FlagDriveAPIState>,
    Path(username): Path<String>,
) -> Response {
    let Ok(followers) = get_followers_list(&api_state.pool, &username).await else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Database error retrieving followers" }).to_string(),
            ))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "followers": followers }).to_string()))
        .unwrap()
}

pub async fn get_following_action(
    State(api_state): State<FlagDriveAPIState>,
    Path(username): Path<String>,
) -> Response {
    let Ok(following) = get_following_list(&api_state.pool, &username).await else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(
                json!({ "error": "Database error retrieving following list" }).to_string(),
            ))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "following": following }).to_string()))
        .unwrap()
}
