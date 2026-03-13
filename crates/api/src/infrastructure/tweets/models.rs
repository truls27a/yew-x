use sqlx::FromRow;

#[derive(FromRow)]
pub struct TweetRow {
    pub id: String,
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
    pub like_count: i64,
    pub liked: bool,
}
