use yew::prelude::*;

use super::types::Notification;
use crate::hooks::QueryState;

#[hook]
pub fn use_notifications() -> QueryState<Vec<Notification>> {
    crate::hooks::use_query("notifications", super::api::get_notifications)
}
