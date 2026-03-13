use super::types::{LoginRequest, MeResponse, RegisterRequest, TokenPair};
use crate::shared::api_client;

pub fn get_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("access_token").ok()?
}

pub fn get_refresh_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("refresh_token").ok()?
}

pub fn save_tokens(access_token: &str, refresh_token: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("access_token", access_token);
            let _ = storage.set_item("refresh_token", refresh_token);
        }
    }
}

pub fn clear_tokens() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("access_token");
            let _ = storage.remove_item("refresh_token");
        }
    }
}

pub async fn login(req: LoginRequest) -> Result<TokenPair, String> {
    let pair = api_client::post::<_, TokenPair>("/api/auth/login", Some(&req)).await?;
    save_tokens(&pair.access_token, &pair.refresh_token);
    Ok(pair)
}

pub async fn register(req: RegisterRequest) -> Result<TokenPair, String> {
    let pair = api_client::post::<_, TokenPair>("/api/auth/register", Some(&req)).await?;
    save_tokens(&pair.access_token, &pair.refresh_token);
    Ok(pair)
}

pub async fn get_me() -> Result<MeResponse, String> {
    api_client::get::<MeResponse>("/api/auth/me").await
}
