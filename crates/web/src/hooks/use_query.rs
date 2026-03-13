use yew::prelude::*;

use super::query_client::{cache_get, cache_set, use_query_client};

#[derive(Clone, Debug, PartialEq)]
pub enum QueryState<T: Clone + PartialEq> {
    Loading,
    Ready(T),
    Error(String),
}

#[hook]
pub fn use_query<T, F, Fut>(key: &str, fetcher: F) -> QueryState<T>
where
    T: Clone + PartialEq + 'static,
    F: Fn() -> Fut + 'static,
    Fut: std::future::Future<Output = Result<T, String>> + 'static,
{
    let client = use_query_client();
    let generation = client.generation();

    let state = use_state({
        let key = key.to_string();
        move || {
            cache_get::<T>(&key)
                .map(QueryState::Ready)
                .unwrap_or(QueryState::Loading)
        }
    });

    let key = key.to_string();

    {
        let state = state.clone();
        let cache_key = key.clone();
        use_effect_with((key, generation), move |_| {
            if let Some(data) = cache_get::<T>(&cache_key) {
                state.set(QueryState::Ready(data));
            } else {
                let state = state.clone();
                let cache_key = cache_key.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetcher().await {
                        Ok(data) => {
                            cache_set(&cache_key, data.clone());
                            state.set(QueryState::Ready(data));
                        }
                        Err(err) => state.set(QueryState::Error(err)),
                    }
                });
            }
        });
    }

    (*state).clone()
}
