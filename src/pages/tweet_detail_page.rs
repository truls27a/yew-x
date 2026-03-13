use yew::prelude::*;
use yew_router::prelude::*;

use crate::features::tweet::components::TweetDetailView;
use crate::features::tweet::hooks::use_tweet_detail;
use crate::hooks::QueryState;
use crate::router::Route;

#[function_component(TweetDetailPage)]
pub fn tweet_detail_page() -> Html {
    let route = use_route::<Route>().unwrap();
    let tweet_id = match route {
        Route::TweetDetail { id } => id,
        _ => String::new(),
    };

    let navigator = use_navigator().unwrap();
    let on_back = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.back();
        })
    };

    let query = use_tweet_detail(tweet_id);

    html! {
        <div>
            <div class="px-4 py-3 border-b border-gray-800 sticky top-0 bg-black bg-opacity-80 backdrop-blur-sm z-10 flex items-center gap-4">
                <button onclick={on_back} class="text-white hover:bg-gray-900 rounded-full p-2 transition-colors">
                    <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                    </svg>
                </button>
                <h1 class="text-xl font-bold text-white">{ "Post" }</h1>
            </div>
            {
                match query {
                    QueryState::Loading => html! {
                        <div class="flex justify-center p-8">
                            <span class="text-gray-500">{ "Loading..." }</span>
                        </div>
                    },
                    QueryState::Ready(None) => html! {
                        <div class="flex justify-center p-8">
                            <span class="text-gray-500">{ "Tweet not found" }</span>
                        </div>
                    },
                    QueryState::Ready(Some(tweet)) => html! {
                        <TweetDetailView tweet={tweet} />
                    },
                }
            }
        </div>
    }
}
