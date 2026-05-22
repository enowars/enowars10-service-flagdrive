mod api_routes;
mod cli;
mod database;

use api_routes::{
    files::{download_file, get_file_list, upload_file},
    gdpr::gdpr_download_user_data,
    gdpr::gdpr_request_user_data,
    login::login_as_user,
    register::register_new_user,
    user::{
        follow_user_action, get_followers_action, get_following_action, get_user_info,
        unfollow_user_action,
    },
};
use axum::{
    Json, Router,
    routing::{get, post},
};
use clap::Parser;
use cli::Args;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let database_url = format!("sqlite://{}", args.database);
    let pool = database::connect_to_db(&database_url).await;

    let flag_drive_api_state = FlagDriveAPIState { pool };

    // Core endpoints
    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/auth/register", post(register_new_user))
        .route("/api/auth/login", post(login_as_user))
        .route("/api/user/{username}", get(get_user_info))
        .route("/api/user/{username}/follow", post(follow_user_action))
        .route("/api/user/{username}/unfollow", post(unfollow_user_action))
        .route("/api/user/{username}/followers", get(get_followers_action))
        .route("/api/user/{username}/following", get(get_following_action))
        .route("/api/gdpr/request", post(gdpr_request_user_data))
        .route(
            "/api/gdpr/download/{user_link}",
            get(gdpr_download_user_data),
        )
        .route("/api/files/{username}", get(get_file_list))
        .route("/api/file/upload", post(upload_file))
        .route("/api/file/download/{file_id}", get(download_file).post(download_file))
        .with_state(flag_drive_api_state)
        // TODO: remove CORS later
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(&args.addr).await.unwrap();
    println!("Backend running on http://{}", args.addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
pub struct FlagDriveAPIState {
    pool: SqlitePool,
}

async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "time": SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs() as u64)
                    .unwrap_or(0)
    }))
}
