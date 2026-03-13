use crate::application::comments::ports::CommentRepository;
use crate::domain::comments::entities::Comment;
use crate::domain::error::AppError;
use crate::domain::users::entities::User;
use crate::infrastructure::shared::unit_of_work::SharedTx;

use super::models::CommentRow;

#[derive(Clone)]
pub struct SqliteCommentRepository {
    tx: SharedTx,
}

impl SqliteCommentRepository {
    pub fn new(tx: SharedTx) -> Self {
        Self { tx }
    }
}

fn row_to_comment(row: CommentRow) -> Comment {
    Comment {
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
        created_at: row.created_at,
    }
}

const COMMENT_QUERY_BASE: &str = "
    SELECT
        c.id, c.tweet_id, c.user_id, c.content, c.created_at,
        u.display_name, u.handle, u.avatar_url, u.bio, u.followers, u.following
    FROM comments c
    JOIN users u ON c.user_id = u.id
";

impl CommentRepository for SqliteCommentRepository {
    async fn find_by_tweet_id(&self, tweet_id: &str) -> Result<Vec<Comment>, AppError> {
        let query = format!("{COMMENT_QUERY_BASE} WHERE c.tweet_id = ? ORDER BY c.created_at ASC");
        let mut tx = self.tx.lock().await;
        let rows: Vec<CommentRow> = sqlx::query_as(&query)
            .bind(tweet_id)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(rows.into_iter().map(row_to_comment).collect())
    }

    async fn create(
        &self,
        id: &str,
        tweet_id: &str,
        user_id: &str,
        content: &str,
    ) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("INSERT INTO comments (id, tweet_id, user_id, content) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(tweet_id)
            .bind(user_id)
            .bind(content)
            .execute(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Comment>, AppError> {
        let query = format!("{COMMENT_QUERY_BASE} WHERE c.id = ?");
        let mut tx = self.tx.lock().await;
        let row: Option<CommentRow> = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(row.map(row_to_comment))
    }
}
