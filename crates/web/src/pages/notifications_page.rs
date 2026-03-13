use yew::prelude::*;
use yew_router::prelude::*;

use crate::features::auth::hooks::use_me;
use crate::features::notifications::components::NotificationItem;
use crate::features::notifications::hooks::use_notifications;
use crate::hooks::QueryState;
use crate::router::Route;

#[function_component(NotificationsPage)]
pub fn notifications_page() -> Html {
    let me = use_me();
    let navigator = use_navigator().unwrap();

    if matches!(&me, QueryState::Error(_)) {
        navigator.push(&Route::Login);
        return html! {};
    }

    if !matches!(&me, QueryState::Ready(_)) {
        return html! {
            <div class="flex justify-center p-8">
                <span class="text-gray-500">{ "Loading..." }</span>
            </div>
        };
    }

    html! { <NotificationsContent /> }
}

#[function_component(NotificationsContent)]
fn notifications_content() -> Html {
    let query = use_notifications();

    html! {
        <div>
            <div class="px-4 py-3 border-b border-gray-800 sticky top-0 bg-black bg-opacity-80 backdrop-blur-sm z-10">
                <h1 class="text-xl font-bold text-white">{ "Notifications" }</h1>
            </div>
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
                    QueryState::Ready(notifications) => html! {
                        { for notifications.iter().map(|notif| html! {
                            <NotificationItem key={notif.id.clone()} notification={notif.clone()} />
                        })}
                    },
                }
            }
        </div>
    }
}
