use super::types::Notification;
use crate::shared::api_client;

pub async fn get_notifications() -> Result<Vec<Notification>, String> {
    api_client::get::<Vec<Notification>>("/api/notifications").await
}
