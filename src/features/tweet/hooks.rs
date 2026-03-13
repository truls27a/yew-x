use yew::prelude::*;

use crate::features::feed::api;
use crate::features::feed::types::Tweet;
use crate::hooks::QueryState;

#[hook]
pub fn use_tweet_detail(id: String) -> QueryState<Option<Tweet>> {
    crate::hooks::use_query(&format!("tweet-detail-{}", id), move || {
        api::get_tweet_by_id(&id)
    })
}
