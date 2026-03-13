use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::hooks::{use_comments, use_create_comment, use_toggle_like};
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

    let comments = use_comments(&tweet.id);
    let comment_content = use_state(String::new);
    let create_comment = use_create_comment(&tweet.id);

    let on_comment_input = {
        let comment_content = comment_content.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            comment_content.set(target.value());
        })
    };

    let on_comment_submit = {
        let comment_content = comment_content.clone();
        let create_comment = create_comment.clone();
        Callback::from(move |_: MouseEvent| {
            let text = (*comment_content).clone();
            if text.trim().is_empty() || create_comment.loading {
                return;
            }
            create_comment.mutate.emit(text);
            comment_content.set(String::new());
        })
    };

    let comment_is_empty = comment_content.trim().is_empty();

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
            // Comment compose form
            {
                if is_logged_in {
                    html! {
                        <div class="flex gap-3 px-0 py-3 border-b border-gray-800">
                            <div class="flex-1">
                                <textarea
                                    value={(*comment_content).clone()}
                                    oninput={on_comment_input}
                                    placeholder="Post your reply"
                                    class="w-full bg-transparent text-white placeholder-gray-600 resize-none outline-none min-h-[60px]"
                                    rows="2"
                                />
                                <div class="flex justify-end pt-2">
                                    <button onclick={on_comment_submit}
                                            disabled={comment_is_empty || create_comment.loading}
                                            class="bg-blue-500 hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed text-white font-bold px-4 py-1.5 rounded-full text-sm transition-colors">
                                        { if create_comment.loading { "Replying..." } else { "Reply" } }
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="py-4 border-b border-gray-800 text-gray-500 text-center">
                            <Link<Route> to={Route::Login} classes="text-blue-500 hover:underline">
                                { "Log in to reply" }
                            </Link<Route>>
                        </div>
                    }
                }
            }
            // Comments list
            {
                match comments {
                    QueryState::Loading => html! {
                        <div class="flex justify-center p-4">
                            <span class="text-gray-500">{ "Loading replies..." }</span>
                        </div>
                    },
                    QueryState::Error(err) => html! {
                        <div class="flex justify-center p-4">
                            <span class="text-red-500">{ format!("Error: {err}") }</span>
                        </div>
                    },
                    QueryState::Ready(comments) => html! {
                        <div>
                            { for comments.iter().map(|comment| {
                                html! {
                                    <div class="flex gap-3 py-3 border-b border-gray-800">
                                        <img src={comment.user.avatar_url.clone()}
                                             class="w-8 h-8 rounded-full flex-shrink-0"
                                             alt={comment.user.handle.clone()} />
                                        <div class="flex-1 min-w-0">
                                            <div class="flex items-center gap-1">
                                                <span class="font-bold text-white text-sm truncate">{ &comment.user.display_name }</span>
                                                <span class="text-gray-500 text-sm truncate">{ format!("@{}", comment.user.handle) }</span>
                                                <span class="text-gray-500 text-sm">{ "\u{00b7}" }</span>
                                                <span class="text-gray-500 text-sm">{ &comment.timestamp }</span>
                                            </div>
                                            <p class="text-white mt-1 whitespace-pre-wrap">{ &comment.content }</p>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    },
                }
            }
        </div>
    }
}
