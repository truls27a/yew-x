use crate::application::notifications::ports::NotificationRepository;
use crate::domain::error::AppError;
use crate::domain::notifications::entities::{Notification, NotificationType};
use crate::infrastructure::shared::unit_of_work::SharedTx;

use super::models::NotificationRow;

#[derive(Clone)]
pub struct SqliteNotificationRepository {
    tx: SharedTx,
}

impl SqliteNotificationRepository {
    pub fn new(tx: SharedTx) -> Self {
        Self { tx }
    }
}

impl NotificationRepository for SqliteNotificationRepository {
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Notification>, AppError> {
        let mut tx = self.tx.lock().await;
        let rows: Vec<NotificationRow> = sqlx::query_as(
            "SELECT n.id, n.notification_type, u.display_name as actor_name, u.handle as actor_handle, u.avatar_url as actor_avatar, n.content, n.created_at
             FROM notifications n
             JOIN users u ON n.actor_id = u.id
             WHERE n.target_user_id = ?
             ORDER BY n.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&mut **tx)
        .await
        .map_err(|e| AppError::Internal {
            message: "Database error".into(),
            source: Some(Box::new(e)),
        })?;

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
                    created_at: r.created_at,
                }
            })
            .collect())
    }
}
