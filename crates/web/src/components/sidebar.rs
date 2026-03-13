use yew::prelude::*;
use yew_router::prelude::*;

use super::icons;
use crate::features::auth::api;
use crate::features::auth::hooks::use_me;
use crate::hooks::QueryState;
use crate::router::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let me = use_me();
    let navigator = use_navigator().unwrap();
    let client = crate::hooks::use_query_client();

    let on_logout = {
        let navigator = navigator.clone();
        let client = client.clone();
        Callback::from(move |_: MouseEvent| {
            api::clear_tokens();
            client.invalidate("me");
            navigator.push(&Route::Login);
        })
    };

    let is_logged_in = matches!(&me, QueryState::Ready(_));

    html! {
        <nav class="flex flex-col gap-2 p-4 sticky top-0">
            <div class="text-blue-500 text-2xl font-bold mb-6 px-3">
                { "Y" }
            </div>
            <Link<Route> to={Route::Home}
                classes="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                <icons::HomeIcon />
                <span>{ "Home" }</span>
            </Link<Route>>
            if is_logged_in {
                <Link<Route> to={Route::Notifications}
                    classes="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                    <icons::BellIcon />
                    <span>{ "Notifications" }</span>
                </Link<Route>>
                if let QueryState::Ready(user) = &me {
                    <Link<Route> to={Route::Profile { id: user.id.clone() }}
                        classes="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                        <icons::UserIcon />
                        <span>{ "Profile" }</span>
                    </Link<Route>>
                }
                <button onclick={on_logout}
                    class="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                    <icons::LogoutIcon />
                    <span>{ "Logout" }</span>
                </button>
            } else {
                <Link<Route> to={Route::Login}
                    classes="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                    <icons::UserIcon />
                    <span>{ "Login" }</span>
                </Link<Route>>
            }
        </nav>
    }
}
