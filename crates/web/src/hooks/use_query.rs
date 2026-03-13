use yew::prelude::*;

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
    let state = use_state(|| QueryState::<T>::Loading);
    let key = key.to_string();

    {
        let state = state.clone();
        use_effect_with(key, move |_key| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetcher().await {
                    Ok(data) => state.set(QueryState::Ready(data)),
                    Err(err) => state.set(QueryState::Error(err)),
                }
            });
            || ()
        });
    }

    (*state).clone()
}
