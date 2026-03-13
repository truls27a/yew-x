use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::api;
use super::types::Tweet;
use crate::components::icons;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct TweetCardProps {
    pub tweet: Tweet,
}

#[function_component(TweetCard)]
pub fn tweet_card(props: &TweetCardProps) -> Html {
    let tweet = &props.tweet;
    let liked = use_state(|| tweet.liked);
    let like_count = use_state(|| tweet.likes);

    let on_like_click = {
        let liked = liked.clone();
        let like_count = like_count.clone();
        let tweet_id = tweet.id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let liked = liked.clone();
            let like_count = like_count.clone();
            let tweet_id = tweet_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = api::toggle_like(&tweet_id).await {
                    liked.set(resp.liked);
                    like_count.set(resp.count);
                }
            });
        })
    };

    let navigator = use_navigator().unwrap();
    let tweet_id = tweet.id.clone();
    let on_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::TweetDetail {
                id: tweet_id.clone(),
            });
        })
    };

    let profile_id = tweet.user.id.clone();
    let on_avatar_click = {
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            navigator.push(&Route::Profile {
                id: profile_id.clone(),
            });
        })
    };

    let heart_class = if *liked {
        "text-pink-600"
    } else {
        "text-gray-500 hover:text-pink-600"
    };

    html! {
        <div onclick={on_click}
             class="flex gap-3 px-4 py-3 border-b border-gray-800 hover:bg-gray-950 cursor-pointer transition-colors">
            <img src={tweet.user.avatar_url.clone()}
                 onclick={on_avatar_click}
                 class="w-10 h-10 rounded-full flex-shrink-0 cursor-pointer hover:opacity-80"
                 alt={tweet.user.handle.clone()} />
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-1">
                    <span class="font-bold text-white truncate">{ &tweet.user.display_name }</span>
                    <span class="text-gray-500 truncate">{ format!("@{}", tweet.user.handle) }</span>
                    <span class="text-gray-500">{ "·" }</span>
                    <span class="text-gray-500">{ &tweet.timestamp }</span>
                </div>
                <p class="text-white mt-1 whitespace-pre-wrap">{ &tweet.content }</p>
                <div class="flex gap-6 mt-3 text-gray-500">
                    <button class="flex items-center gap-1 hover:text-blue-500 transition-colors">
                        <icons::ReplyIcon />
                        <span class="text-sm">{ tweet.replies }</span>
                    </button>
                    <button class="flex items-center gap-1 hover:text-green-500 transition-colors">
                        <icons::RetweetIcon />
                        <span class="text-sm">{ tweet.retweets }</span>
                    </button>
                    <button onclick={on_like_click}
                            class={classes!("flex", "items-center", "gap-1", "transition-colors", heart_class)}>
                        <icons::HeartIcon filled={*liked} />
                        <span class="text-sm">{ *like_count }</span>
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ComposeTweetProps {
    pub on_submit: Callback<Tweet>,
}

#[function_component(ComposeTweet)]
pub fn compose_tweet(props: &ComposeTweetProps) -> Html {
    let content = use_state(String::new);
    let submitting = use_state(|| false);

    let on_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(target.value());
        })
    };

    let on_submit = {
        let content = content.clone();
        let submitting = submitting.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |_: MouseEvent| {
            let text = (*content).clone();
            if text.trim().is_empty() || *submitting {
                return;
            }
            let content = content.clone();
            let submitting = submitting.clone();
            let on_submit = on_submit.clone();
            submitting.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_tweet("current", &text).await {
                    Ok(tweet) => {
                        on_submit.emit(tweet);
                        content.set(String::new());
                    }
                    Err(err) => {
                        log::error!("Failed to create tweet: {}", err);
                    }
                }
                submitting.set(false);
            });
        })
    };

    let is_empty = content.trim().is_empty();

    html! {
        <div class="flex gap-3 px-4 py-3 border-b border-gray-800">
            <img src="https://i.pravatar.cc/150?u=you"
                 class="w-10 h-10 rounded-full flex-shrink-0"
                 alt="Your avatar" />
            <div class="flex-1">
                <textarea
                    value={(*content).clone()}
                    oninput={on_input}
                    placeholder="What's happening?"
                    class="w-full bg-transparent text-white text-xl placeholder-gray-600 resize-none outline-none min-h-[80px]"
                    rows="3"
                />
                <div class="flex justify-end border-t border-gray-800 pt-3">
                    <button onclick={on_submit}
                            disabled={is_empty || *submitting}
                            class="bg-blue-500 hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed text-white font-bold px-5 py-2 rounded-full transition-colors">
                        { if *submitting { "Posting..." } else { "Post" } }
                    </button>
                </div>
            </div>
        </div>
    }
}
