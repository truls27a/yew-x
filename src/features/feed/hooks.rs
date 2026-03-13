use yew::prelude::*;

use super::api;
use super::types::Tweet;
use crate::hooks::QueryState;

#[hook]
pub fn use_tweets() -> (QueryState<Vec<Tweet>>, UseStateHandle<Vec<Tweet>>) {
    let local_tweets = use_state(Vec::<Tweet>::new);
    let query = crate::hooks::use_query("tweets", || api::get_all_tweets());
    (query, local_tweets)
}

#[hook]
pub fn use_tweet(id: String) -> QueryState<Option<Tweet>> {
    crate::hooks::use_query(&format!("tweet-{}", id), move || {
        api::get_tweet_by_id(&id)
    })
}
