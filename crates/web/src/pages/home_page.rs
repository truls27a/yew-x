use yew::prelude::*;

use crate::features::tweets::components::{ComposeTweet, TweetCard};
use crate::features::tweets::hooks::use_tweets;
use crate::hooks::QueryState;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let query = use_tweets();

    html! {
        <div>
            <div class="px-4 py-3 border-b border-gray-800 sticky top-0 bg-black bg-opacity-80 backdrop-blur-sm z-10">
                <h1 class="text-xl font-bold text-white">{ "Home" }</h1>
            </div>
            <ComposeTweet />
            {
                match query {
                    QueryState::Loading => html! {
                        <div class="flex justify-center p-8">
                            <span class="text-gray-500">{ "Loading..." }</span>
                        </div>
                    },
                    QueryState::Error(err) => html! {
                        <div class="flex justify-center p-8">
                            <span class="text-red-500">{ format!("Error: {err}") }</span>
                        </div>
                    },
                    QueryState::Ready(tweets) => html! {
                        { for tweets.iter().map(|tweet| html! {
                            <TweetCard key={tweet.id.clone()} tweet={tweet.clone()} />
                        })}
                    },
                }
            }
        </div>
    }
}
