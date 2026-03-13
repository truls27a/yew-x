use yew::prelude::*;

use crate::features::feed::types::{Tweet, User};
use crate::hooks::QueryState;

#[hook]
pub fn use_profile(user_id: String) -> QueryState<Option<(User, Vec<Tweet>)>> {
    crate::hooks::use_query(&format!("profile-{}", user_id), move || {
        super::api::get_profile(&user_id)
    })
}
