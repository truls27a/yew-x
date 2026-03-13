use yew::prelude::*;

use super::api;
use super::types::Tweet;
use crate::hooks::{use_mutation, use_query_client, Mutation, QueryState};

#[hook]
pub fn use_tweets() -> QueryState<Vec<Tweet>> {
    crate::hooks::use_query("tweets", || api::get_all_tweets())
}

#[hook]
pub fn use_create_tweet() -> Mutation<String> {
    let client = use_query_client();
    use_mutation(move |content: String| {
        let client = client.clone();
        async move {
            let result = api::create_tweet(&content).await;
            if result.is_ok() {
                client.invalidate("tweet");
            }
            result
        }
    })
}

#[hook]
pub fn use_toggle_like(
    tweet_id: &str,
    initial_liked: bool,
    initial_count: u32,
) -> (bool, u32, Callback<MouseEvent>) {
    let client = use_query_client();
    let liked = use_state(|| initial_liked);
    let count = use_state(|| initial_count);

    let toggle = {
        let liked = liked.clone();
        let count = count.clone();
        let client = client.clone();
        let tweet_id = tweet_id.to_string();
        Callback::from(move |_: MouseEvent| {
            let liked = liked.clone();
            let count = count.clone();
            let client = client.clone();
            let tweet_id = tweet_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = api::toggle_like(&tweet_id).await {
                    liked.set(resp.liked);
                    count.set(resp.count);
                    client.invalidate("tweet");
                }
            });
        })
    };

    (*liked, *count, toggle)
}
