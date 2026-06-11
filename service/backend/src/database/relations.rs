use sqlx::sqlite::SqlitePool;
use sqlx::Row;

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
