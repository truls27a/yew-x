use super::user::User;

#[derive(Clone, Debug, PartialEq)]
pub struct Tweet {
    pub id: String,
    pub user: User,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub likes: u32,
    pub retweets: u32,
    pub replies: u32,
    pub liked: bool,
    pub retweeted: bool,
}
