use flagdrive_shared::{FlagDriveFile, FlagDriveFileVisibility};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn get_user_files(
    pool: &SqlitePool,
    username: &str,
    viewer: Option<&str>,
) -> Result<Vec<FlagDriveFile>, sqlx::Error> {
    let viewer_str = viewer.unwrap_or("");

    let rows = sqlx::query(
        "SELECT id, name, owner, visibility, size, created_at, protection_key, is_protected FROM files \
         WHERE ( \
             owner = $1 \
             OR (owner IN (SELECT followee FROM follows WHERE follower = $1) AND (visibility = 1 OR visibility = 3)) \
             OR (owner IN (SELECT follower FROM follows WHERE followee = $1) AND visibility = 2) \
         ) \
         AND ( \
             visibility = 1 \
             OR owner = $2 \
             OR (owner IN (SELECT followee FROM follows WHERE follower = $2) AND (visibility = 1 OR visibility = 3)) \
             OR (owner IN (SELECT follower FROM follows WHERE followee = $2) AND visibility = 2) \
         )"
    )
    .bind(username)
    .bind(viewer_str)
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
            is_protected: row.get::<i64, _>("is_protected") != 0,
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
    protection_key: &str,
    is_protected: i32,
) -> Result<(), sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    let size = content.len();

    sqlx::query(
        "INSERT INTO files (id, name, owner, visibility, size, content, created_at, protection_key, is_protected) \
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(name)
    .bind(owner)
    .bind(visibility)
    .bind(size as i64)
    .bind(content)
    .bind(now as i64)
    .bind(protection_key)
    .bind(is_protected)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_download_file(
    pool: &SqlitePool,
    id: i64,
) -> Result<(FlagDriveFile, Vec<u8>, String), sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, name, owner, visibility, size, content, created_at, protection_key, is_protected FROM files \
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
            is_protected: row.get::<i64, _>("is_protected") != 0,
        },
        row.get("content"),
        row.get("protection_key"),
    ))
}
