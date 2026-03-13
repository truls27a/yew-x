use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub handle: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers: u32,
    pub following: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub user: User,
    pub content: String,
    pub timestamp: String,
    pub likes: u32,
    pub retweets: u32,
    pub replies: u32,
    pub liked: bool,
    pub retweeted: bool,
}

pub fn current_user() -> User {
    User {
        id: "current".to_string(),
        display_name: "You".to_string(),
        handle: "you".to_string(),
        avatar_url: "https://i.pravatar.cc/150?u=you".to_string(),
        bio: "Building things with Yew and Rust.".to_string(),
        followers: 42,
        following: 108,
    }
}
