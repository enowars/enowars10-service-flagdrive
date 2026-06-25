use rand::prelude::*;
use sqlx::postgres::PgPool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub enum DbPool {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

pub async fn connect_to_db(database_url: &str) -> DbPool {
    let pool =
        if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
            let pg_pool = PgPool::connect(database_url)
                .await
                .expect("Failed to connect to PostgreSQL database");
            DbPool::Postgres(pg_pool)
        } else {
            let connection_options = SqliteConnectOptions::from_str(database_url)
                .unwrap()
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(Duration::from_secs(10))
                .foreign_keys(true);

            let sqlite_pool = SqlitePool::connect_with(connection_options)
                .await
                .expect("Failed to connect to SQLite database");
            DbPool::Sqlite(sqlite_pool)
        };

    match &pool {
        DbPool::Sqlite(sqlite_pool) => {
            sqlx::migrate!("./migrations/sqlite")
                .run(sqlite_pool)
                .await
                .expect("Failed to run SQLite database migrations");
        }
        DbPool::Postgres(pg_pool) => {
            sqlx::migrate!("./migrations/postgres")
                .run(pg_pool)
                .await
                .expect("Failed to run PostgreSQL database migrations");
        }
    }

    pool
}

pub async fn delete_old_data(pool: &DbPool, age_limit_seconds: u64) -> Result<u64, sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u64)
        .unwrap_or(0);

    let cutoff = now.saturating_sub(age_limit_seconds) as i64;

    let rows_affected = match pool {
        DbPool::Sqlite(p) => {
            let result = sqlx::query("DELETE FROM users WHERE created_at < $1")
                .bind(cutoff)
                .execute(p)
                .await?;
            result.rows_affected()
        }
        DbPool::Postgres(p) => {
            let result = sqlx::query("DELETE FROM users WHERE created_at < $1")
                .bind(cutoff)
                .execute(p)
                .await?;
            let rows = result.rows_affected();
            if rows > 0 {
                let _ = sqlx::query("VACUUM").execute(p).await;
            }
            rows
        }
    };

    Ok(rows_affected)
}

pub async fn get_or_create_server_key(pool: &DbPool) -> Result<String, sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            let row = sqlx::query("SELECT value FROM server_config WHERE key = 'server_key'")
                .fetch_optional(p)
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

                sqlx::query("INSERT INTO server_config (key, value) VALUES ('server_key', $1)")
                    .bind(&new_key)
                    .execute(p)
                    .await?;

                Ok(new_key)
            }
        }
        DbPool::Postgres(p) => {
            let row = sqlx::query("SELECT value FROM server_config WHERE key = 'server_key'")
                .fetch_optional(p)
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

                sqlx::query("INSERT INTO server_config (key, value) VALUES ('server_key', $1)")
                    .bind(&new_key)
                    .execute(p)
                    .await?;

                Ok(new_key)
            }
        }
    }
}
