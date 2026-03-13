use gloo_net::http::Request;

use super::types::{LoginRequest, MeResponse, RegisterRequest, TokenPair};

const API_BASE: &str = "http://localhost:3000/api";

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
    let resp = Request::post(&format!("{API_BASE}/auth/login"))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err("Invalid email or password".to_string());
    }

    let pair: TokenPair = resp.json().await.map_err(|e| e.to_string())?;
    save_tokens(&pair.access_token, &pair.refresh_token);
    Ok(pair)
}

pub async fn register(req: RegisterRequest) -> Result<TokenPair, String> {
    let resp = Request::post(&format!("{API_BASE}/auth/register"))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err("Registration failed".to_string());
    }

    let pair: TokenPair = resp.json().await.map_err(|e| e.to_string())?;
    save_tokens(&pair.access_token, &pair.refresh_token);
    Ok(pair)
}

pub async fn get_me() -> Result<MeResponse, String> {
    let token = get_token().ok_or_else(|| "Not authenticated".to_string())?;
    let resp = Request::get(&format!("{API_BASE}/auth/me"))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() != 200 {
        return Err("Not authenticated".to_string());
    }

    resp.json::<MeResponse>()
        .await
        .map_err(|e| e.to_string())
}
