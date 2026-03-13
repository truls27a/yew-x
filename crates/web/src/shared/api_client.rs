use gloo_net::http::{Request, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::features::auth::api::{clear_tokens, get_refresh_token, get_token, save_tokens};

const API_BASE: &str = "http://localhost:3000";

#[derive(Serialize)]
struct RefreshRequestBody {
    refresh_token: String,
}

#[derive(Deserialize)]
struct RefreshResponseBody {
    access_token: String,
    refresh_token: String,
}

async fn request<B: Serialize, T: DeserializeOwned>(
    make_builder: impl Fn(&str) -> RequestBuilder,
    path: &str,
    body: Option<&B>,
) -> Result<T, String> {
    let url = format!("{API_BASE}{path}");

    let send = |token: Option<String>| {
        let mut builder = make_builder(&url);
        if let Some(token) = token {
            builder = builder.header("Authorization", &format!("Bearer {token}"));
        }
        async {
            if let Some(body) = body {
                builder.json(body).map_err(|e| e.to_string())?.send().await
            } else {
                builder.send().await
            }
            .map_err(|e| e.to_string())
        }
    };

    let resp = send(get_token()).await?;

    if resp.status() == 401 {
        let refresh_token = get_refresh_token().ok_or("Not authenticated".to_string())?;

        let refresh_resp = Request::post(&format!("{API_BASE}/api/auth/refresh"))
            .json(&RefreshRequestBody { refresh_token })
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if refresh_resp.status() != 200 {
            clear_tokens();
            return Err("Not authenticated".to_string());
        }

        let tokens: RefreshResponseBody =
            refresh_resp.json().await.map_err(|e| e.to_string())?;
        save_tokens(&tokens.access_token, &tokens.refresh_token);

        let resp = send(get_token()).await?;
        return resp.json::<T>().await.map_err(|e| e.to_string());
    }

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
