use crate::features::feed::api;
use crate::features::feed::types::{Comment, Tweet};
use crate::hooks::{use_mutation, use_query_client, Mutation, QueryState};

pub use crate::features::feed::hooks::use_toggle_like;

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

#[yew::prelude::hook]
pub fn use_comments(tweet_id: &str) -> QueryState<Vec<Comment>> {
    let tweet_id = tweet_id.to_string();
    crate::hooks::use_query(&format!("comments-{}", tweet_id), move || {
        let tweet_id = tweet_id.clone();
        async move { api::get_comments(&tweet_id).await }
    })
}

#[yew::prelude::hook]
pub fn use_create_comment(tweet_id: &str) -> Mutation<String> {
    let client = use_query_client();
    let tweet_id = tweet_id.to_string();
    use_mutation(move |content: String| {
        let client = client.clone();
        let tweet_id = tweet_id.clone();
        async move {
            let result = api::create_comment(&tweet_id, &content).await;
            if result.is_ok() {
                client.invalidate("comment");
                client.invalidate("tweet");
            }
            result
        }
    })
}
