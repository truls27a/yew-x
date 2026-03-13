use sqlx::SqlitePool;

use crate::application::ports::user_repository::UserRepository;
use crate::domain::entities::user::User;
use crate::infrastructure::models::UserRow;

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
    async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<User>> {
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
}
