use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::application::notifications::ports::NotificationRepository;
use crate::domain::notifications::entities::{Notification, NotificationType};
use super::models::NotificationRow;

#[derive(Clone)]
pub struct SqliteNotificationRepository {
    pool: SqlitePool,
}

impl SqliteNotificationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl NotificationRepository for SqliteNotificationRepository {
    async fn find_by_user_id(&self, user_id: &str) -> anyhow::Result<Vec<Notification>> {
        let rows: Vec<NotificationRow> = sqlx::query_as(
            "SELECT n.id, n.notification_type, u.display_name as actor_name, u.handle as actor_handle, u.avatar_url as actor_avatar, n.content, n.created_at
             FROM notifications n
             JOIN users u ON n.actor_id = u.id
             WHERE n.target_user_id = ?
             ORDER BY n.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let notification_type = match r.notification_type.as_str() {
                    "Like" => NotificationType::Like,
                    "Retweet" => NotificationType::Retweet,
                    "Follow" => NotificationType::Follow,
                    "Reply" => NotificationType::Reply,
                    _ => NotificationType::Like,
                };
                Notification {
                    id: r.id,
                    notification_type,
                    actor_name: r.actor_name,
                    actor_handle: r.actor_handle,
                    actor_avatar: r.actor_avatar,
                    content: r.content,
                    created_at: NaiveDateTime::parse_from_str(&r.created_at, "%Y-%m-%d %H:%M:%S")
                        .unwrap_or_default(),
                }
            })
            .collect())
    }
}
