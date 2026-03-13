use yew::prelude::*;
use yew_router::prelude::*;

use crate::features::tweets::components::TweetCard;
use crate::features::users::components::ProfileHeader;
use crate::features::users::hooks::use_profile;
use crate::hooks::QueryState;
use crate::router::Route;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let route = use_route::<Route>().unwrap();
    let user_id = match route {
        Route::Profile { id } => id,
        _ => "current".to_string(),
    };

    let query = use_profile(user_id);

    html! {
        <div>
            <div class="px-4 py-3 border-b border-gray-800 sticky top-0 bg-black bg-opacity-80 backdrop-blur-sm z-10">
                <h1 class="text-xl font-bold text-white">{ "Profile" }</h1>
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
                    QueryState::Ready(None) => html! {
                        <div class="flex justify-center p-8">
                            <span class="text-gray-500">{ "User not found" }</span>
                        </div>
                    },
                    QueryState::Ready(Some((user, tweets))) => html! {
                        <>
                            <ProfileHeader user={user} />
                            { for tweets.iter().map(|tweet| html! {
                                <TweetCard key={tweet.id.clone()} tweet={tweet.clone()} />
                            })}
                        </>
                    },
                }
            }
        </div>
    }
}
