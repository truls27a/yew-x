use serde::{Deserialize, Serialize};

use crate::domain::notifications::entities::{Notification, NotificationType};
use crate::domain::tweets::entities::Tweet;
use crate::domain::users::entities::User;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub display_name: String,
    pub handle: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers: u32,
    pub following: u32,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            display_name: u.display_name,
            handle: u.handle,
            avatar_url: u.avatar_url,
            bio: u.bio,
            followers: u.followers,
            following: u.following,
        }
    }
}

#[derive(Serialize)]
pub struct TweetResponse {
    pub id: String,
    pub user: UserResponse,
    pub content: String,
    pub timestamp: String,
    pub likes: u32,
    pub retweets: u32,
    pub replies: u32,
    pub liked: bool,
    pub retweeted: bool,
}

impl From<Tweet> for TweetResponse {
    fn from(t: Tweet) -> Self {
        Self {
            id: t.id,
            user: UserResponse::from(t.user),
            content: t.content,
            timestamp: format_timestamp(t.created_at),
            likes: t.likes,
            retweets: t.retweets,
            replies: t.replies,
            liked: t.liked,
            retweeted: t.retweeted,
        }
    }
}

#[derive(Serialize)]
pub struct NotificationResponse {
    pub id: String,
    pub notification_type: String,
    pub actor_name: String,
    pub actor_handle: String,
    pub actor_avatar: String,
    pub content: Option<String>,
    pub timestamp: String,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        let notification_type = match n.notification_type {
            NotificationType::Like => "Like",
            NotificationType::Retweet => "Retweet",
            NotificationType::Follow => "Follow",
            NotificationType::Reply => "Reply",
        }
        .to_string();

        Self {
            id: n.id,
            notification_type,
            actor_name: n.actor_name,
            actor_handle: n.actor_handle,
            actor_avatar: n.actor_avatar,
            content: n.content,
            timestamp: format_timestamp(n.created_at),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTweetRequest {
    pub content: String,
}

#[derive(Serialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub count: u32,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct TokenPairResponse {
    pub access_token: String,
    pub refresh_token: String,
}

fn format_timestamp(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default()
}
