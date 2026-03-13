use chrono::NaiveDateTime;

use crate::application::tweets::ports::TweetRepository;
use crate::domain::error::AppError;
use crate::domain::tweets::entities::Tweet;
use crate::domain::users::entities::User;
use crate::infrastructure::shared::unit_of_work::SharedTx;

use super::models::TweetRow;

#[derive(Clone)]
pub struct SqliteTweetRepository {
    tx: SharedTx,
}

impl SqliteTweetRepository {
    pub fn new(tx: SharedTx) -> Self {
        Self { tx }
    }
}

fn row_to_tweet(row: TweetRow) -> Tweet {
    Tweet {
        id: row.id,
        user: User {
            id: row.user_id,
            display_name: row.display_name,
            handle: row.handle,
            avatar_url: row.avatar_url,
            bio: row.bio,
            followers: row.followers as u32,
            following: row.following as u32,
        },
        content: row.content,
        created_at: NaiveDateTime::parse_from_str(&row.created_at, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_default(),
        likes: row.like_count as u32,
        retweets: 0,
        replies: 0,
        liked: row.liked,
        retweeted: false,
    }
}

const TWEET_QUERY_BASE: &str = "
    SELECT
        t.id, t.user_id, t.content, t.created_at,
        u.display_name, u.handle, u.avatar_url, u.bio, u.followers, u.following,
        COALESCE((SELECT COUNT(*) FROM tweet_likes WHERE tweet_id = t.id), 0) as like_count,
        EXISTS(SELECT 1 FROM tweet_likes WHERE tweet_id = t.id AND user_id = ?) as liked
    FROM tweets t
    JOIN users u ON t.user_id = u.id
";

impl TweetRepository for SqliteTweetRepository {
    async fn find_all(&self, current_user_id: Option<&str>) -> Result<Vec<Tweet>, AppError> {
        let query = format!("{TWEET_QUERY_BASE} ORDER BY t.created_at DESC");
        let mut tx = self.tx.lock().await;
        let rows: Vec<TweetRow> = sqlx::query_as(&query)
            .bind(current_user_id.unwrap_or(""))
            .fetch_all(&mut **tx)
            .await?;
        Ok(rows.into_iter().map(row_to_tweet).collect())
    }

    async fn find_by_id(
        &self,
        id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Option<Tweet>, AppError> {
        let query = format!("{TWEET_QUERY_BASE} WHERE t.id = ?");
        let mut tx = self.tx.lock().await;
        let row: Option<TweetRow> = sqlx::query_as(&query)
            .bind(current_user_id.unwrap_or(""))
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?;
        Ok(row.map(row_to_tweet))
    }

    async fn find_by_user_id(
        &self,
        user_id: &str,
        current_user_id: Option<&str>,
    ) -> Result<Vec<Tweet>, AppError> {
        let query = format!("{TWEET_QUERY_BASE} WHERE t.user_id = ? ORDER BY t.created_at DESC");
        let mut tx = self.tx.lock().await;
        let rows: Vec<TweetRow> = sqlx::query_as(&query)
            .bind(current_user_id.unwrap_or(""))
            .bind(user_id)
            .fetch_all(&mut **tx)
            .await?;
        Ok(rows.into_iter().map(row_to_tweet).collect())
    }

    async fn create(&self, id: &str, user_id: &str, content: &str) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("INSERT INTO tweets (id, user_id, content) VALUES (?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(content)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn toggle_like(&self, tweet_id: &str, user_id: &str) -> Result<(bool, u32), AppError> {
        let mut tx = self.tx.lock().await;
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tweet_likes WHERE tweet_id = ? AND user_id = ?)")
                .bind(tweet_id)
                .bind(user_id)
                .fetch_one(&mut **tx)
                .await?;

        if exists {
            sqlx::query("DELETE FROM tweet_likes WHERE tweet_id = ? AND user_id = ?")
                .bind(tweet_id)
                .bind(user_id)
                .execute(&mut **tx)
                .await?;
        } else {
            sqlx::query("INSERT INTO tweet_likes (tweet_id, user_id) VALUES (?, ?)")
                .bind(tweet_id)
                .bind(user_id)
                .execute(&mut **tx)
                .await?;
        }

        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM tweet_likes WHERE tweet_id = ?")
                .bind(tweet_id)
                .fetch_one(&mut **tx)
                .await?;

        Ok((!exists, count.0 as u32))
    }
}
