use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use super::types::{Tweet, User};

const API_BASE: &str = "http://localhost:3000/api";

pub async fn get_all_tweets() -> Result<Vec<Tweet>, String> {
    Request::get(&format!("{API_BASE}/tweets"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Tweet>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_tweet_by_id(tweet_id: &str) -> Result<Tweet, String> {
    Request::get(&format!("{API_BASE}/tweets/{tweet_id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Tweet>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_user_by_id(user_id: &str) -> Result<User, String> {
    Request::get(&format!("{API_BASE}/users/{user_id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<User>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_tweets_by_user(user_id: &str) -> Result<Vec<Tweet>, String> {
    Request::get(&format!("{API_BASE}/users/{user_id}/tweets"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Tweet>>()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct CreateTweetBody {
    user_id: String,
    content: String,
}

pub async fn create_tweet(user_id: &str, content: &str) -> Result<Tweet, String> {
    Request::post(&format!("{API_BASE}/tweets"))
        .json(&CreateTweetBody {
            user_id: user_id.to_string(),
            content: content.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Tweet>()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Deserialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub count: u32,
}

pub async fn toggle_like(tweet_id: &str) -> Result<LikeResponse, String> {
    Request::post(&format!("{API_BASE}/tweets/{tweet_id}/like"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<LikeResponse>()
        .await
        .map_err(|e| e.to_string())
}
