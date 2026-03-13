use crate::application::users::ports::UserRepository;
use crate::domain::error::AppError;
use crate::domain::users::entities::User;
use crate::infrastructure::shared::unit_of_work::SharedTx;

use super::models::UserRow;

#[derive(Clone)]
pub struct SqliteUserRepository {
    tx: SharedTx,
}

impl SqliteUserRepository {
    pub fn new(tx: SharedTx) -> Self {
        Self { tx }
    }
}

impl UserRepository for SqliteUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let mut tx = self.tx.lock().await;
        let row: Option<UserRow> =
            sqlx::query_as("SELECT id, display_name, handle, avatar_url, bio, followers, following FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .map_err(|e| AppError::Internal {
                    message: "Database error".into(),
                    source: Some(Box::new(e)),
                })?;

        Ok(row.map(|r| User {
            id: r.id,
            display_name: r.display_name,
            handle: r.handle,
            avatar_url: r.avatar_url,
            bio: r.bio,
            followers: r.followers as u32,
            following: r.following as u32,
        }))
    }

    async fn create(
        &self,
        id: &str,
        display_name: &str,
        handle: &str,
        avatar_url: &str,
    ) -> Result<(), AppError> {
        let mut tx = self.tx.lock().await;
        sqlx::query("INSERT INTO users (id, display_name, handle, avatar_url) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(display_name)
            .bind(handle)
            .bind(avatar_url)
            .execute(&mut **tx)
            .await
            .map_err(|e| AppError::Internal {
                message: "Database error".into(),
                source: Some(Box::new(e)),
            })?;
        Ok(())
    }
}
