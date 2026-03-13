use crate::features::feed::api;
use crate::features::feed::types::Tweet;
use crate::hooks::QueryState;

#[yew::prelude::hook]
pub fn use_tweet_detail(id: String) -> QueryState<Option<Tweet>> {
    crate::hooks::use_query(&format!("tweet-detail-{}", id), move || {
        let id = id.clone();
        async move {
            match api::get_tweet_by_id(&id).await {
                Ok(tweet) => Ok(Some(tweet)),
                Err(_) => Ok(None),
            }
        }
    })
}
