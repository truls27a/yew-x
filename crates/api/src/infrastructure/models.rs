use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserRow {
    pub id: String,
    pub display_name: String,
    pub handle: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers: i64,
    pub following: i64,
}

#[derive(FromRow)]
pub struct TweetRow {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
    // Joined fields
    pub display_name: String,
    pub handle: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers: i64,
    pub following: i64,
    pub like_count: i64,
    pub liked: bool,
}

#[derive(FromRow)]
pub struct NotificationRow {
    pub id: String,
    pub notification_type: String,
    pub actor_name: String,
    pub actor_handle: String,
    pub actor_avatar: String,
    pub content: Option<String>,
    pub created_at: String,
}
