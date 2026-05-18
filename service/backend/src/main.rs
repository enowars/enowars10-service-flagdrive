// backend/src/main.rs
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Core endpoints
    let app = Router::new()
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/dashboard", get(dashboard_handler))
        // TODO: remove CORS later
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4859").await.unwrap();
    println!("Backend running on http://0.0.0.0:4859");
    axum::serve(listener, app).await.unwrap();
}

async fn register_handler() -> Json<Value> {
    // TODO: Hash password, save to DB
    let raw_json_stub = r#"{
        "token": "secure-token-123",
        "user": {
            "username": "NewUser_From_Backend"
        }
    }"#;

    let response = serde_json::from_str(raw_json_stub)
        .expect("Backend failed to parse register stub");

    Json(response)
}

async fn login_handler() -> Json<Value> {
    // TODO: Verify password against DB
    let raw_json_stub = r#"{
        "token": "secure-token-123",
        "user": {
            "username": "NewUser_From_Backend"
        }
    }"#;

    let response = serde_json::from_str(raw_json_stub)
        .expect("Backend failed to parse register stub");

    Json(response)
}

async fn dashboard_handler() -> Json<Value> {
    // TODO: Extract Token from Auth header and fetch user
    let raw_json_stub = r#"{
        "content": "stub data"
    }"#;

    let response = serde_json::from_str(raw_json_stub)
        .expect("Backend failed to parse register stub");

    Json(response)
}