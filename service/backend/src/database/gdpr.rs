use crate::database::DbPool;
use sqlx::Row;

pub async fn insert_gdpr_data(
    pool: &DbPool,
    username: &str,
    timestamp: i64,
    nonce: &str,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO gdpr_data (username, timestamp, nonce, content) VALUES ($1, $2, $3, $4)")
        .bind(username)
        .bind(timestamp)
        .bind(nonce)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_gdpr_data(
    pool: &DbPool,
    username: &str,
    timestamp_or_latest: &str,
    nonce: &str,
) -> Result<String, sqlx::Error> {
    if nonce.len() != 32 {
        return Err(sqlx::Error::RowNotFound);
    }

    let row = if timestamp_or_latest == "latest" {
        sqlx::query(
            "SELECT content FROM gdpr_data WHERE username = $1 AND nonce = $2 \
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
            "SELECT content FROM gdpr_data WHERE username = $1 AND timestamp = $2 AND nonce = $3",
        )
        .bind(username)
        .bind(timestamp)
        .bind(nonce)
        .fetch_one(pool)
        .await?
    };
    Ok(row.get("content"))
}
