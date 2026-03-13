use yew::prelude::*;

use crate::features::feed::types::User;

#[derive(Properties, PartialEq)]
pub struct ProfileHeaderProps {
    pub user: User,
}

#[function_component(ProfileHeader)]
pub fn profile_header(props: &ProfileHeaderProps) -> Html {
    let user = &props.user;

    html! {
        <div>
            // Banner
            <div class="h-48 bg-gray-800"></div>
            // Avatar + info
            <div class="px-4">
                <div class="relative -mt-16 mb-3">
                    <img src={user.avatar_url.clone()}
                         class="w-32 h-32 rounded-full border-4 border-black"
                         alt={user.handle.clone()} />
                </div>
                <h2 class="text-xl font-bold text-white">{ &user.display_name }</h2>
                <p class="text-gray-500">{ format!("@{}", user.handle) }</p>
                <p class="text-white mt-3">{ &user.bio }</p>
                <div class="flex gap-4 mt-3 text-sm">
                    <span>
                        <span class="font-bold text-white">{ user.following }</span>
                        <span class="text-gray-500">{ " Following" }</span>
                    </span>
                    <span>
                        <span class="font-bold text-white">{ user.followers }</span>
                        <span class="text-gray-500">{ " Followers" }</span>
                    </span>
                </div>
            </div>
            <div class="border-b border-gray-800 mt-4"></div>
        </div>
    }
}
