use super::types::Comment;
use crate::hooks::{use_mutation, use_query_client, Mutation, QueryState};

#[yew::prelude::hook]
pub fn use_comments(tweet_id: &str) -> QueryState<Vec<Comment>> {
    let tweet_id = tweet_id.to_string();
    crate::hooks::use_query(&format!("comments-{}", tweet_id), move || {
        let tweet_id = tweet_id.clone();
        async move { super::api::get_comments(&tweet_id).await }
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
            let result = super::api::create_comment(&tweet_id, &content).await;
            if result.is_ok() {
                client.invalidate("comment");
                client.invalidate("tweet");
            }
            result
        }
    })
}
