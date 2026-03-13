use serde::Serialize;

use super::types::Comment;
use crate::shared::api_client;

pub async fn get_comments(tweet_id: &str) -> Result<Vec<Comment>, String> {
    api_client::get::<Vec<Comment>>(&format!("/api/tweets/{tweet_id}/comments")).await
}

#[derive(Serialize)]
struct CreateCommentBody {
    content: String,
}

pub async fn create_comment(tweet_id: &str, content: &str) -> Result<Comment, String> {
    api_client::post::<_, Comment>(
        &format!("/api/tweets/{tweet_id}/comments"),
        Some(&CreateCommentBody {
            content: content.to_string(),
        }),
    )
    .await
}
