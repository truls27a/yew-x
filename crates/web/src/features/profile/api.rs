use crate::features::feed::api::{get_tweets_by_user, get_user_by_id};
use crate::features::feed::types::{Tweet, User};

pub async fn get_profile(user_id: &str) -> Result<Option<(User, Vec<Tweet>)>, String> {
    let user = match get_user_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return Ok(None),
    };
    let tweets = get_tweets_by_user(user_id).await?;
    Ok(Some((user, tweets)))
}
