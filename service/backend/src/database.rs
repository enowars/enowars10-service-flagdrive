use flagdrive_shared::{FlagDriveFile, FlagDriveFileVisibility, FlagDriveUser};
use rand::prelude::*;
use sqlx::Row;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn connect_to_db(database_url: &str) -> SqlitePool {
    let connection_options = SqliteConnectOptions::from_str(database_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connection_options)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

pub async fn _delete_old_data(
    pool: &SqlitePool,
    age_limit_seconds: u64,
) -> Result<u64, sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    let cutoff = now.saturating_sub(age_limit_seconds) as i64;

    let result = sqlx::query("DELETE FROM files WHERE created_at < ?")
        .bind(cutoff)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

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
         (SELECT COUNT(*) FROM follows WHERE followee = users.username AND follower = ?) as is_followed \
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

pub async fn insert_gdpr_data(
    pool: &SqlitePool,
    username: &str,
    timestamp: i64,
    nonce: &str,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO gdpr_data (username, timestamp, nonce, content) VALUES (?, ?, ?, ?)")
        .bind(username)
        .bind(timestamp)
        .bind(nonce)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_gdpr_data(
    pool: &SqlitePool,
    username: &str,
    timestamp_or_latest: &str,
    nonce: &str,
) -> Result<String, sqlx::Error> {
    let row = if timestamp_or_latest == "latest" {
        sqlx::query(
            "SELECT content FROM gdpr_data WHERE username = ? AND nonce LIKE ? \
             ORDER BY timestamp DESC LIMIT 1",
        )
        .bind(username)
        .bind(nonce)
        .fetch_one(pool)
        .await?
    } else {
        let timestamp: i64 = timestamp_or_latest
            .parse()
            .map_err(|_| sqlx::Error::RowNotFound)?;

        sqlx::query(
            "SELECT content FROM gdpr_data WHERE username = ? AND timestamp = ? AND nonce LIKE ?",
        )
        .bind(username)
        .bind(timestamp)
        .bind(nonce)
        .fetch_one(pool)
        .await?
    };

    Ok(row.get("content"))
}

pub async fn follow_user(
    pool: &SqlitePool,
    follower: &str,
    followee: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR IGNORE INTO follows (follower, followee) VALUES (?, ?)")
        .bind(follower)
        .bind(followee)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn is_following(
    pool: &SqlitePool,
    follower: &str,
    followee: &str,
) -> Result<bool, sqlx::Error> {
    let row =
        sqlx::query("SELECT EXISTS(SELECT 1 FROM follows WHERE follower = ? AND followee = ?)")
            .bind(follower)
            .bind(followee)
            .fetch_one(pool)
            .await?;
    let exists: i32 = row.get(0);
    Ok(exists != 0)
}

pub async fn unfollow_user(
    pool: &SqlitePool,
    follower: &str,
    followee: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM follows WHERE follower = ? AND followee = ?")
        .bind(follower)
        .bind(followee)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_following_list(
    pool: &SqlitePool,
    username: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT followee FROM follows WHERE follower = ?")
        .bind(username)
        .fetch_all(pool)
        .await?;
    let following = rows.into_iter().map(|row| row.get("followee")).collect();
    Ok(following)
}

pub async fn get_followers_list(
    pool: &SqlitePool,
    username: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT follower FROM follows WHERE followee = ?")
        .bind(username)
        .fetch_all(pool)
        .await?;
    let followers = rows.into_iter().map(|row| row.get("follower")).collect();
    Ok(followers)
}

pub async fn get_user_files(
    pool: &SqlitePool,
    username: &str,
) -> Result<Vec<FlagDriveFile>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, name, owner, visibility, size, created_at, encryption_key FROM files \
         WHERE owner = $1 \
         OR (owner IN (SELECT followee FROM follows WHERE follower = $1) AND (visibility = 1 OR visibility = 3)) \
         OR (owner IN (SELECT follower FROM follows WHERE followee = $1) AND (visibility = 2))"
    )
    .bind(username)
    .fetch_all(pool)
    .await?;

    let files = rows
        .into_iter()
        .map(|row| FlagDriveFile {
            id: row.get::<i64, _>("id") as u64,
            name: row.get("name"),
            owner: row.get("owner"),
            visibility: match row.get("visibility") {
                1 => FlagDriveFileVisibility::Public,
                2 => FlagDriveFileVisibility::Following,
                3 => FlagDriveFileVisibility::Followers,
                _ => FlagDriveFileVisibility::Private,
            },
            size: row.get::<i64, _>("size") as u64,
            created_at: row.get::<i64, _>("created_at") as u64,
            is_encrypted: !row.get::<String, _>("encryption_key").is_empty(),
        })
        .collect();

    Ok(files)
}

pub async fn add_upload_file(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    owner: &str,
    visibility: i32,
    content: &[u8],
    encryption_key: &str,
) -> Result<(), sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    let size = content.len();

    sqlx::query(
        "INSERT INTO files (id, name, owner, visibility, size, content, created_at, encryption_key) \
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(name)
    .bind(owner)
    .bind(visibility)
    .bind(size as i64)
    .bind(content)
    .bind(now as i64)
    .bind(encryption_key)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_download_file(
    pool: &SqlitePool,
    id: i64,
) -> Result<(FlagDriveFile, Vec<u8>, String), sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, name, owner, visibility, size, content, created_at, encryption_key FROM files \
         WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok((
        FlagDriveFile {
            id: row.get::<i64, _>("id") as u64,
            name: row.get("name"),
            owner: row.get("owner"),
            visibility: match row.get("visibility") {
                1 => FlagDriveFileVisibility::Public,
                2 => FlagDriveFileVisibility::Following,
                3 => FlagDriveFileVisibility::Followers,
                _ => FlagDriveFileVisibility::Private,
            },
            size: row.get::<i64, _>("size") as u64,
            created_at: row.get::<i64, _>("created_at") as u64,
            is_encrypted: !row.get::<String, _>("encryption_key").is_empty(),
        },
        row.get("content"),
        row.get("encryption_key"),
    ))
}
