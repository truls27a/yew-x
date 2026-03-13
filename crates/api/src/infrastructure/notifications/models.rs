use sqlx::FromRow;

#[derive(FromRow)]
pub struct NotificationRow {
    pub id: String,
    pub notification_type: String,
    pub actor_name: String,
    pub actor_handle: String,
    pub actor_avatar: String,
    pub content: Option<String>,
    pub created_at: i64,
}
