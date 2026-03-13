use gloo_net::http::{Request, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};

use crate::features::auth::api::get_token;

const API_BASE: &str = "http://localhost:3000";

async fn request<B: Serialize, T: DeserializeOwned>(
    make_builder: impl Fn(&str) -> RequestBuilder,
    path: &str,
    body: Option<&B>,
) -> Result<T, String> {
    let url = format!("{API_BASE}{path}");

    let mut builder = make_builder(&url);
    if let Some(token) = get_token() {
        builder = builder.header("Authorization", &format!("Bearer {token}"));
    }

    let resp = if let Some(body) = body {
        builder.json(body).map_err(|e| e.to_string())?.send().await
    } else {
        builder.send().await
    }
    .map_err(|e| e.to_string())?;

    resp.json::<T>().await.map_err(|e| e.to_string())
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    request::<(), T>(Request::get, path, None).await
}

pub async fn post<B: Serialize, T: DeserializeOwned>(
    path: &str,
    body: Option<&B>,
) -> Result<T, String> {
    request(Request::post, path, body).await
}

pub async fn put<B: Serialize, T: DeserializeOwned>(
    path: &str,
    body: Option<&B>,
) -> Result<T, String> {
    request(Request::put, path, body).await
}

pub async fn patch<B: Serialize, T: DeserializeOwned>(
    path: &str,
    body: Option<&B>,
) -> Result<T, String> {
    request(Request::patch, path, body).await
}

pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    request::<(), T>(Request::delete, path, None).await
}
