use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::Layout;
use crate::hooks::QueryClientProvider;
use crate::pages::home_page::HomePage;
use crate::pages::notifications_page::NotificationsPage;
use crate::pages::profile_page::ProfilePage;
use crate::pages::tweet_detail_page::TweetDetailPage;
use crate::router::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Profile { id: _ } => html! { <ProfilePage /> },
        Route::TweetDetail { id: _ } => html! { <TweetDetailPage /> },
        Route::Notifications => html! { <NotificationsPage /> },
        Route::NotFound => html! {
            <div class="flex justify-center items-center h-64">
                <span class="text-gray-500 text-xl">{ "404 — Page not found" }</span>
            </div>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <QueryClientProvider>
            <BrowserRouter>
                <Layout>
                    <Switch<Route> render={switch} />
                </Layout>
            </BrowserRouter>
        </QueryClientProvider>
    }
}
