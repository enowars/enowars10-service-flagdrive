use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use serde_json::{Value, json};
use shared::{File, FileVisibility, User};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Core endpoints
    let app = Router::new()
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/files", get(files_handler))
        .route("/api/user/{username}", get(user_handler))
        // TODO: remove CORS later
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4859").await.unwrap();
    println!("Backend running on http://0.0.0.0:4859");
    axum::serve(listener, app).await.unwrap();
}

async fn register_handler(Json(payload): Json<Value>) -> Json<Value> {
    // TODO: Hash password, save to DB
    let _username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let _password = payload
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    Json(json!({
        "token": "secure-token-123",
        "user": {
            "username": _username
        }
    }))
}

async fn login_handler(Json(payload): Json<Value>) -> Json<Value> {
    // TODO: Verify password against DB
    let _username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let _password = payload
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    Json(json!({
        "token": "secure-token-123",
        "user": {
            "username": _username
        }
    }))
}

async fn files_handler() -> Json<Vec<File>> {
    let files = vec![
        File {
            id: "1".to_string(),
            name: "Tax_Returns_2025.pdf".to_string(),
            owner: "citizen_492".to_string(),
            visibility: FileVisibility::Private,
            size: 1048576,
        },
        File {
            id: "2".to_string(),
            name: "Public_Health_Guidelines.txt".to_string(),
            owner: "dept_of_health".to_string(),
            visibility: FileVisibility::Public,
            size: 2048,
        },
        File {
            id: "3".to_string(),
            name: "Internal_Audit_Q3.zip".to_string(),
            owner: "inspector_general".to_string(),
            visibility: FileVisibility::Following,
            size: 536870912,
        },
        File {
            id: "4".to_string(),
            name: "Press_Release_Draft.docx".to_string(),
            owner: "media_office".to_string(),
            visibility: FileVisibility::Followers,
            size: 45000,
        },
    ];

    Json(files)
}

async fn user_handler(Path(username): Path<String>) -> Json<User> {
    let user = User {
        username,
        followers_count: 15,
        following_count: 8,
        is_followed: false,
    };

    Json(user)
}
