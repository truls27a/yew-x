use sqlx::FromRow;

#[derive(FromRow)]
pub struct CommentRow {
    pub id: String,
    pub tweet_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: i64,
    // Joined fields
    pub display_name: String,
    pub handle: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers: i64,
    pub following: i64,
}
