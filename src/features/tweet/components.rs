use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::icons;
use crate::features::feed::types::Tweet;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct TweetDetailProps {
    pub tweet: Tweet,
}

#[function_component(TweetDetailView)]
pub fn tweet_detail_view(props: &TweetDetailProps) -> Html {
    let tweet = &props.tweet;
    let liked = use_state(|| tweet.liked);
    let like_count = use_state(|| tweet.likes);

    let on_like_click = {
        let liked = liked.clone();
        let like_count = like_count.clone();
        Callback::from(move |_: MouseEvent| {
            if *liked {
                like_count.set(*like_count - 1);
            } else {
                like_count.set(*like_count + 1);
            }
            liked.set(!*liked);
        })
    };

    let navigator = use_navigator().unwrap();
    let profile_id = tweet.user.id.clone();
    let on_avatar_click = {
        Callback::from(move |_: MouseEvent| {
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
                    <span class="font-bold text-white">{ *like_count }</span>
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
                <button onclick={on_like_click}
                        class={classes!("transition-colors", "p-2", heart_class)}>
                    <icons::HeartIcon filled={*liked} />
                </button>
            </div>
        </div>
    }
}
