use super::types::Notification;
use crate::shared::api::client;

pub async fn get_notifications() -> Result<Vec<Notification>, String> {
    client::get::<Vec<Notification>>("/api/notifications").await
}
