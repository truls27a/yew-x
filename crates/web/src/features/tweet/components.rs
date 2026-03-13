use yew::prelude::*;
use yew_router::prelude::*;

use super::hooks::use_toggle_like;
use crate::components::icons;
use crate::features::auth::hooks::use_me;
use crate::features::feed::types::Tweet;
use crate::hooks::QueryState;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct TweetDetailProps {
    pub tweet: Tweet,
}

#[function_component(TweetDetailView)]
pub fn tweet_detail_view(props: &TweetDetailProps) -> Html {
    let tweet = &props.tweet;
    let me = use_me();
    let (liked, like_count, on_like_click) =
        use_toggle_like(&tweet.id, tweet.liked, tweet.likes);

    let is_logged_in = matches!(&me, QueryState::Ready(_));
    let navigator = use_navigator().unwrap();

    let profile_id = tweet.user.id.clone();
    let on_avatar_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Profile {
                id: profile_id.clone(),
            });
        })
    };

    let on_like_wrapper = {
        let on_like_click = on_like_click.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            if is_logged_in {
                on_like_click.emit(e);
            } else {
                navigator.push(&Route::Login);
            }
        })
    };

    let heart_class = if liked {
        "text-pink-600"
    } else {
        "text-gray-500 hover:text-pink-600"
    };

    html! {
        <div class="px-4 py-3">
            <div class="flex gap-3 items-center">
                <img src={tweet.user.avatar_url.clone()}
                     onclick={on_avatar_click}
                     class="w-12 h-12 rounded-full cursor-pointer hover:opacity-80"
                     alt={tweet.user.handle.clone()} />
                <div>
                    <div class="font-bold text-white">{ &tweet.user.display_name }</div>
                    <div class="text-gray-500">{ format!("@{}", tweet.user.handle) }</div>
                </div>
            </div>
            <p class="text-white text-2xl mt-4 whitespace-pre-wrap">{ &tweet.content }</p>
            <div class="text-gray-500 mt-4 py-4 border-b border-gray-800">
                { &tweet.timestamp }
            </div>
            <div class="flex gap-6 py-4 border-b border-gray-800 text-gray-500">
                <span>
                    <span class="font-bold text-white">{ tweet.retweets }</span>
                    { " Retweets" }
                </span>
                <span>
                    <span class="font-bold text-white">{ like_count }</span>
                    { " Likes" }
                </span>
            </div>
            <div class="flex gap-6 py-3 border-b border-gray-800 text-gray-500 justify-around">
                <button class="hover:text-blue-500 transition-colors p-2">
                    <icons::ReplyIcon />
                </button>
                <button class="hover:text-green-500 transition-colors p-2">
                    <icons::RetweetIcon />
                </button>
                <button onclick={on_like_wrapper}
                        class={classes!("transition-colors", "p-2", heart_class)}>
                    <icons::HeartIcon filled={liked} />
                </button>
            </div>
        </div>
    }
}
