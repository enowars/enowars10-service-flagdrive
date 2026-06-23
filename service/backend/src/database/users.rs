use flagdrive_shared::FlagDriveUser;
use rand::prelude::*;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn create_new_user(
    pool: &SqlitePool,
    username: &str,
    user_password: &str,
    encryption_key: &str,
) -> Result<(), sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    sqlx::query("INSERT INTO users (username, user_password, created_at, encryption_key) VALUES (?, ?, ?, ?)")
        .bind(username)
        .bind(user_password)
        .bind(now as i64)
        .bind(encryption_key)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn create_new_token(pool: &SqlitePool, username: &str) -> Result<String, sqlx::Error> {
    let token = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect::<String>();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    sqlx::query("INSERT INTO auth_token (username, token, created_at) VALUES (?, ?, ?)")
        .bind(username)
        .bind(token.clone())
        .bind(now as i64)
        .execute(pool)
        .await?;

    Ok(token)
}

pub async fn check_user_password(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT user_password FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get::<String, _>("user_password") == password)
}

pub async fn get_user_by_username(
    pool: &SqlitePool,
    username: &str,
    viewer: Option<&str>,
) -> Result<FlagDriveUser, sqlx::Error> {
    let viewer_str = viewer.unwrap_or("");

    let row = sqlx::query(
        "SELECT \
         username, \
         (SELECT COUNT(*) FROM follows WHERE followee = users.username) as followers_count, \
         (SELECT COUNT(*) FROM follows WHERE follower = users.username) as following_count, \
         (SELECT EXISTS(SELECT 1 FROM follows WHERE followee = users.username AND follower = ?)) as is_followed \
         FROM users WHERE username = ?",
    )
    .bind(viewer_str)
    .bind(username)
    .fetch_one(pool)
    .await?;

    let username_str: String = row.get("username");
    let followers: i64 = row.get("followers_count");
    let following: i64 = row.get("following_count");
    let is_followed_count: i64 = row.get("is_followed");

    Ok(FlagDriveUser {
        username: username_str,
        followers_count: followers as usize,
        following_count: following as usize,
        is_followed: is_followed_count > 0,
    })
}

pub async fn get_username_from_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT username FROM auth_token WHERE token = ?")
        .bind(token)
        .fetch_one(pool)
        .await?;

    Ok(row.get("username"))
}

pub async fn get_user_encryption_key(
    pool: &SqlitePool,
    username: &str,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT encryption_key FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("encryption_key"))
}

pub async fn delete_token(pool: &SqlitePool, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM auth_token WHERE token = ?")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}
