use gloo_net::http::Request;
use serde::Serialize;

use super::types::Comment;
use crate::features::auth::api::get_token;

const API_BASE: &str = "http://localhost:3000/api";

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
