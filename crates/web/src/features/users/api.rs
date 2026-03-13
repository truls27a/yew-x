use super::types::User;
use crate::features::tweets::api::get_tweets_by_user;
use crate::features::tweets::types::Tweet;
use crate::shared::api::client;

pub async fn get_user_by_id(user_id: &str) -> Result<User, String> {
    client::get::<User>(&format!("/api/users/{user_id}")).await
}

pub async fn get_profile(user_id: &str) -> Result<Option<(User, Vec<Tweet>)>, String> {
    let user = match get_user_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return Ok(None),
    };
    let tweets = get_tweets_by_user(user_id).await?;
    Ok(Some((user, tweets)))
}
