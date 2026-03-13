use crate::features::feed::api::{get_tweets_by_user, get_user_by_id};
use crate::features::feed::types::{Tweet, User};

pub fn get_profile(user_id: &str) -> Option<(User, Vec<Tweet>)> {
    get_user_by_id(user_id).map(|user| {
        let tweets = get_tweets_by_user(user_id);
        (user, tweets)
    })
}
