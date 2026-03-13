use gloo_net::http::Request;

use super::types::Notification;
use crate::features::auth::api::get_token;

const API_BASE: &str = "http://localhost:3000/api";

pub async fn get_notifications() -> Result<Vec<Notification>, String> {
    let token = get_token().ok_or_else(|| "Not authenticated".to_string())?;
    Request::get(&format!("{API_BASE}/notifications"))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Notification>>()
        .await
        .map_err(|e| e.to_string())
}
