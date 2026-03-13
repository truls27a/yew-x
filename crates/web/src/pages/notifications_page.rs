use yew::prelude::*;

use crate::features::notifications::components::NotificationItem;
use crate::features::notifications::hooks::use_notifications;
use crate::hooks::QueryState;

#[function_component(NotificationsPage)]
pub fn notifications_page() -> Html {
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
