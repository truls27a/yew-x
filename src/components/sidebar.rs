use yew::prelude::*;
use yew_router::prelude::*;

use super::icons;
use crate::router::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
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
            <Link<Route> to={Route::Notifications}
                classes="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                <icons::BellIcon />
                <span>{ "Notifications" }</span>
            </Link<Route>>
            <Link<Route> to={Route::Profile { id: "current".to_string() }}
                classes="flex items-center gap-4 px-3 py-3 rounded-full hover:bg-gray-900 transition-colors text-xl text-white">
                <icons::UserIcon />
                <span>{ "Profile" }</span>
            </Link<Route>>
        </nav>
    }
}
