use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum QueryState<T: Clone + PartialEq> {
    Loading,
    Ready(T),
}

#[hook]
pub fn use_query<T, F>(key: &str, fetcher: F) -> QueryState<T>
where
    T: Clone + PartialEq + 'static,
    F: Fn() -> T + 'static,
{
    let state = use_state(|| QueryState::<T>::Loading);
    let key = key.to_string();

    {
        let state = state.clone();
        use_effect_with(key, move |_key| {
            let data = fetcher();
            state.set(QueryState::Ready(data));
            || ()
        });
    }

    (*state).clone()
}
