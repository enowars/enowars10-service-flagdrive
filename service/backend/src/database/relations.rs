use crate::database::DbPool;
use sqlx::Row;

pub async fn follow_user(pool: &DbPool, follower: &str, followee: &str) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query(
                "INSERT INTO follows (follower, followee) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(follower)
            .bind(followee)
            .execute(p)
            .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query(
                "INSERT INTO follows (follower, followee) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(follower)
            .bind(followee)
            .execute(p)
            .await?;
        }
    }
    Ok(())
}

pub async fn is_following(
    pool: &DbPool,
    follower: &str,
    followee: &str,
) -> Result<bool, sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            let row = sqlx::query(
                "SELECT EXISTS(SELECT 1 FROM follows WHERE follower = $1 AND followee = $2)",
            )
            .bind(follower)
            .bind(followee)
            .fetch_one(p)
            .await?;
            let exists: i32 = row.get(0);
            Ok(exists != 0)
        }
        DbPool::Postgres(p) => {
            let row = sqlx::query(
                "SELECT EXISTS(SELECT 1 FROM follows WHERE follower = $1 AND followee = $2)",
            )
            .bind(follower)
            .bind(followee)
            .fetch_one(p)
            .await?;
            let exists: bool = row.get(0);
            Ok(exists)
        }
    }
}

pub async fn unfollow_user(
    pool: &DbPool,
    follower: &str,
    followee: &str,
) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("DELETE FROM follows WHERE follower = $1 AND followee = $2")
                .bind(follower)
                .bind(followee)
                .execute(p)
                .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query("DELETE FROM follows WHERE follower = $1 AND followee = $2")
                .bind(follower)
                .bind(followee)
                .execute(p)
                .await?;
        }
    }
    Ok(())
}

pub async fn get_following_list(pool: &DbPool, username: &str) -> Result<Vec<String>, sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            let rows = sqlx::query("SELECT followee FROM follows WHERE follower = $1")
                .bind(username)
                .fetch_all(p)
                .await?;
            let following = rows.into_iter().map(|row| row.get("followee")).collect();
            Ok(following)
        }
        DbPool::Postgres(p) => {
            let rows = sqlx::query("SELECT followee FROM follows WHERE follower = $1")
                .bind(username)
                .fetch_all(p)
                .await?;
            let following = rows.into_iter().map(|row| row.get("followee")).collect();
            Ok(following)
        }
    }
}

pub async fn get_followers_list(pool: &DbPool, username: &str) -> Result<Vec<String>, sqlx::Error> {
    match pool {
        DbPool::Sqlite(p) => {
            let rows = sqlx::query("SELECT follower FROM follows WHERE followee = $1")
                .bind(username)
                .fetch_all(p)
                .await?;
            let followers = rows.into_iter().map(|row| row.get("follower")).collect();
            Ok(followers)
        }
        DbPool::Postgres(p) => {
            let rows = sqlx::query("SELECT follower FROM follows WHERE followee = $1")
                .bind(username)
                .fetch_all(p)
                .await?;
            let followers = rows.into_iter().map(|row| row.get("follower")).collect();
            Ok(followers)
        }
    }
}
