use yew::prelude::*;

use crate::features::feed::components::{ComposeTweet, TweetCard};
use crate::features::feed::hooks::use_tweets;
use crate::features::feed::types::Tweet;
use crate::hooks::QueryState;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (query, local_tweets) = use_tweets();

    let on_submit = {
        let local_tweets = local_tweets.clone();
        Callback::from(move |tweet: Tweet| {
            let mut tweets = (*local_tweets).clone();
            tweets.insert(0, tweet);
            local_tweets.set(tweets);
        })
    };

    html! {
        <div>
            <div class="px-4 py-3 border-b border-gray-800 sticky top-0 bg-black bg-opacity-80 backdrop-blur-sm z-10">
                <h1 class="text-xl font-bold text-white">{ "Home" }</h1>
            </div>
            <ComposeTweet {on_submit} />
            {
                match query {
                    QueryState::Loading => html! {
                        <div class="flex justify-center p-8">
                            <span class="text-gray-500">{ "Loading..." }</span>
                        </div>
                    },
                    QueryState::Ready(tweets) => {
                        let all_tweets: Vec<Tweet> = (*local_tweets).iter().chain(tweets.iter()).cloned().collect();
                        html! {
                            { for all_tweets.iter().map(|tweet| html! {
                                <TweetCard key={tweet.id.clone()} tweet={tweet.clone()} />
                            })}
                        }
                    }
                }
            }
        </div>
    }
}
