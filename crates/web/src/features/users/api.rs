use super::types::User;
use crate::features::tweets::api::get_tweets_by_user;
use crate::features::tweets::types::Tweet;
use gloo_net::http::Request;

const API_BASE: &str = "http://localhost:3000/api";

pub async fn get_user_by_id(user_id: &str) -> Result<User, String> {
    Request::get(&format!("{API_BASE}/users/{user_id}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<User>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_profile(user_id: &str) -> Result<Option<(User, Vec<Tweet>)>, String> {
    let user = match get_user_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return Ok(None),
    };
    let tweets = get_tweets_by_user(user_id).await?;
    Ok(Some((user, tweets)))
}
