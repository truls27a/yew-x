use gloo_net::http::Request;

use super::types::Notification;

const API_BASE: &str = "http://localhost:3000/api";

pub async fn get_notifications() -> Result<Vec<Notification>, String> {
    Request::get(&format!("{API_BASE}/notifications"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Notification>>()
        .await
        .map_err(|e| e.to_string())
}
