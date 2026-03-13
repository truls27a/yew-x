use sqlx::SqlitePool;

use crate::application::users::ports::UserRepository;
use crate::domain::error::AppError;
use crate::domain::users::entities::User;
use super::models::UserRow;

#[derive(Clone)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl UserRepository for SqliteUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let row: Option<UserRow> =
            sqlx::query_as("SELECT id, display_name, handle, avatar_url, bio, followers, following FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

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
        sqlx::query("INSERT INTO users (id, display_name, handle, avatar_url) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(display_name)
            .bind(handle)
            .bind(avatar_url)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
