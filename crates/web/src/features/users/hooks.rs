use super::types::User;
use crate::features::tweets::types::Tweet;
use crate::hooks::QueryState;

#[yew::prelude::hook]
pub fn use_profile(user_id: String) -> QueryState<Option<(User, Vec<Tweet>)>> {
    crate::hooks::use_query(&format!("profile-{}", user_id), move || {
        let user_id = user_id.clone();
        async move { super::api::get_profile(&user_id).await }
    })
}
