use serde::{Deserialize, Serialize};

use crate::domain::entities::notification::{Notification, NotificationType};
use crate::domain::entities::tweet::Tweet;
use crate::domain::entities::user::User;

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
            timestamp: relative_time(t.created_at),
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
            timestamp: relative_time(n.created_at),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTweetRequest {
    pub user_id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub count: u32,
}

fn relative_time(dt: chrono::NaiveDateTime) -> String {
    let now = chrono::Utc::now().naive_utc();
    let diff = now - dt;

    if diff.num_minutes() < 1 {
        "now".to_string()
    } else if diff.num_hours() < 1 {
        format!("{}m", diff.num_minutes())
    } else if diff.num_days() < 1 {
        format!("{}h", diff.num_hours())
    } else {
        format!("{}d", diff.num_days())
    }
}
