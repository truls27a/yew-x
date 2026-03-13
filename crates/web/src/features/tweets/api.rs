use serde::Serialize;

use super::types::Tweet;
use crate::shared::api::client;

pub async fn get_all_tweets() -> Result<Vec<Tweet>, String> {
    client::get::<Vec<Tweet>>("/api/tweets").await
}

pub async fn get_tweet_by_id(tweet_id: &str) -> Result<Tweet, String> {
    client::get::<Tweet>(&format!("/api/tweets/{tweet_id}")).await
}

pub async fn get_tweets_by_user(user_id: &str) -> Result<Vec<Tweet>, String> {
    client::get::<Vec<Tweet>>(&format!("/api/users/{user_id}/tweets")).await
}

#[derive(Serialize)]
struct CreateTweetBody {
    content: String,
}

pub async fn create_tweet(content: &str) -> Result<Tweet, String> {
    client::post::<_, Tweet>(
        "/api/tweets",
        Some(&CreateTweetBody {
            content: content.to_string(),
        }),
    )
    .await
}

#[derive(serde::Deserialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub count: u32,
}

pub async fn toggle_like(tweet_id: &str) -> Result<LikeResponse, String> {
    client::post::<(), LikeResponse>(&format!("/api/tweets/{tweet_id}/like"), None).await
}
