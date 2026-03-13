#[derive(Clone, Debug, PartialEq)]
pub enum NotificationType {
    Like,
    Retweet,
    Follow,
    Reply,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: String,
    pub notification_type: NotificationType,
    pub actor_name: String,
    pub actor_handle: String,
    pub actor_avatar: String,
    pub content: Option<String>,
    pub created_at: i64,
}
