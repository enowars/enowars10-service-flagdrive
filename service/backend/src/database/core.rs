use rand::prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub async fn connect_to_db(database_url: &str) -> SqlitePool {
    let connection_options = SqliteConnectOptions::from_str(database_url)
        .unwrap()
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePool::connect_with(connection_options)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

pub async fn delete_old_data(
    pool: &SqlitePool,
    age_limit_seconds: u64,
) -> Result<u64, sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    let cutoff = now.saturating_sub(age_limit_seconds) as i64;

    let result = sqlx::query("DELETE FROM users WHERE created_at < ?")
        .bind(cutoff)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub async fn get_or_create_server_key(pool: &SqlitePool) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT value FROM server_config WHERE key = 'server_key'")
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        use sqlx::Row;
        Ok(row.get("value"))
    } else {
        let new_key = rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .take(128)
            .map(char::from)
            .collect::<String>();

        sqlx::query("INSERT INTO server_config (key, value) VALUES ('server_key', ?)")
            .bind(&new_key)
            .execute(pool)
            .await?;

        Ok(new_key)
    }
}
