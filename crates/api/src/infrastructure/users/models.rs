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
