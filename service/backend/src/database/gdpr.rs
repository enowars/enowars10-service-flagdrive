use crate::database::DbPool;
use sqlx::Row;

pub async fn insert_gdpr_data(
    pool: &DbPool,
    username: &str,
    timestamp: i64,
    nonce: &str,
    content: &str,
) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO gdpr_data (username, timestamp, nonce, content) VALUES ($1, $2, $3, $4)")
                .bind(username)
                .bind(timestamp)
                .bind(nonce)
                .bind(content)
                .execute(p)
                .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO gdpr_data (username, timestamp, nonce, content) VALUES ($1, $2, $3, $4)")
                .bind(username)
                .bind(timestamp)
                .bind(nonce)
                .bind(content)
                .execute(p)
                .await?;
        }
    }
    Ok(())
}

pub async fn get_gdpr_data(
    pool: &DbPool,
    username: &str,
    timestamp_or_latest: &str,
    nonce: &str,
) -> Result<String, sqlx::Error> {
    let mut nonce_upper = nonce.to_string();
    if let Some(last_char) = nonce_upper.pop() {
        let next_char = (last_char as u8 + 1) as char;
        nonce_upper.push(next_char);
    }

    match pool {
        DbPool::Sqlite(p) => {
            let row = if timestamp_or_latest == "latest" {
                sqlx::query(
                    "SELECT content FROM gdpr_data WHERE username = $1 AND nonce >= $2 AND nonce < $3 \
                     ORDER BY timestamp DESC LIMIT 1",
                )
                .bind(username)
                .bind(nonce)
                .bind(&nonce_upper)
                .fetch_one(p)
                .await?
            } else {
                let timestamp: i64 = timestamp_or_latest
                    .parse()
                    .map_err(|_| sqlx::Error::RowNotFound)?;

                sqlx::query(
                    "SELECT content FROM gdpr_data WHERE username = $1 AND timestamp = $2 AND nonce >= $3 AND nonce < $4",
                )
                .bind(username)
                .bind(timestamp)
                .bind(nonce)
                .bind(&nonce_upper)
                .fetch_one(p)
                .await?
            };
            Ok(row.get("content"))
        }
        DbPool::Postgres(p) => {
            let row = if timestamp_or_latest == "latest" {
                sqlx::query(
                    "SELECT content FROM gdpr_data WHERE username = $1 AND nonce >= $2 AND nonce < $3 \
                     ORDER BY timestamp DESC LIMIT 1",
                )
                .bind(username)
                .bind(nonce)
                .bind(&nonce_upper)
                .fetch_one(p)
                .await?
            } else {
                let timestamp: i64 = timestamp_or_latest
                    .parse()
                    .map_err(|_| sqlx::Error::RowNotFound)?;

                sqlx::query(
                    "SELECT content FROM gdpr_data WHERE username = $1 AND timestamp = $2 AND nonce >= $3 AND nonce < $4",
                )
                .bind(username)
                .bind(timestamp)
                .bind(nonce)
                .bind(&nonce_upper)
                .fetch_one(p)
                .await?
            };
            Ok(row.get("content"))
        }
    }
}
