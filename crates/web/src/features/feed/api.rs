use gloo_net::http::Request;
use serde::Serialize;

use super::types::{Comment, Tweet, User};
use crate::features::auth::api::get_token;

const API_BASE: &str = "http://localhost:3000/api";

pub async fn get_all_tweets() -> Result<Vec<Tweet>, String> {
    let mut req = Request::get(&format!("{API_BASE}/tweets"));
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    req.send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Tweet>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_tweet_by_id(tweet_id: &str) -> Result<Tweet, String> {
    let mut req = Request::get(&format!("{API_BASE}/tweets/{tweet_id}"));
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    req.send()
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
    let mut req = Request::get(&format!("{API_BASE}/users/{user_id}/tweets"));
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    req.send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Tweet>>()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct CreateTweetBody {
    content: String,
}

pub async fn create_tweet(content: &str) -> Result<Tweet, String> {
    let token = get_token().ok_or_else(|| "Not authenticated".to_string())?;
    Request::post(&format!("{API_BASE}/tweets"))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&CreateTweetBody {
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

#[derive(serde::Deserialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub count: u32,
}

pub async fn get_comments(tweet_id: &str) -> Result<Vec<Comment>, String> {
    let mut req = Request::get(&format!("{API_BASE}/tweets/{tweet_id}/comments"));
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    req.send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Comment>>()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct CreateCommentBody {
    content: String,
}

pub async fn create_comment(tweet_id: &str, content: &str) -> Result<Comment, String> {
    let token = get_token().ok_or_else(|| "Not authenticated".to_string())?;
    Request::post(&format!("{API_BASE}/tweets/{tweet_id}/comments"))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&CreateCommentBody {
            content: content.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Comment>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn toggle_like(tweet_id: &str) -> Result<LikeResponse, String> {
    let token = get_token().ok_or_else(|| "Not authenticated".to_string())?;
    Request::post(&format!("{API_BASE}/tweets/{tweet_id}/like"))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<LikeResponse>()
        .await
        .map_err(|e| e.to_string())
}
