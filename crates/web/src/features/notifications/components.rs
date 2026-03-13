use yew::prelude::*;

use super::types::Notification;
use crate::components::icons;

#[derive(Properties, PartialEq)]
pub struct NotificationItemProps {
    pub notification: Notification,
}

#[function_component(NotificationItem)]
pub fn notification_item(props: &NotificationItemProps) -> Html {
    let notif = &props.notification;

    let (icon, color, action_text) = match notif.notification_type.as_str() {
        "Like" => (
            html! { <icons::HeartIcon filled={true} /> },
            "text-pink-600",
            "liked your tweet",
        ),
        "Retweet" => (
            html! { <icons::RetweetIcon /> },
            "text-green-500",
            "retweeted your tweet",
        ),
        "Follow" => (
            html! { <icons::UserIcon /> },
            "text-blue-500",
            "followed you",
        ),
        "Reply" => (
            html! { <icons::ReplyIcon /> },
            "text-blue-500",
            "replied to your tweet",
        ),
        _ => (
            html! { <icons::BellIcon /> },
            "text-gray-500",
            "sent a notification",
        ),
    };

    html! {
        <div class="flex gap-3 px-4 py-3 border-b border-gray-800 hover:bg-gray-950 transition-colors">
            <div class={classes!("flex-shrink-0", "mt-1", color)}>
                { icon }
            </div>
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2">
                    <img src={notif.actor_avatar.clone()}
                         class="w-8 h-8 rounded-full"
                         alt={notif.actor_handle.clone()} />
                </div>
                <p class="mt-1">
                    <span class="font-bold text-white">{ &notif.actor_name }</span>
                    { " " }
                    <span class="text-gray-500">{ action_text }</span>
                    <span class="text-gray-500">{ format!(" · {}", notif.timestamp) }</span>
                </p>
                if let Some(ref content) = notif.content {
                    <p class="text-gray-500 mt-1">{ content }</p>
                }
            </div>
        </div>
    }
}
