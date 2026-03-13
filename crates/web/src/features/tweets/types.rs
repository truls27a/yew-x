use serde::{Deserialize, Serialize};

use crate::features::users::types::User;

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
