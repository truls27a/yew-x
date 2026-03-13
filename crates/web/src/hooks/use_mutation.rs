use yew::prelude::*;

#[derive(Clone)]
pub struct Mutation<A: 'static> {
    pub mutate: Callback<A>,
    pub loading: bool,
}

impl<A: 'static> PartialEq for Mutation<A> {
    fn eq(&self, other: &Self) -> bool {
        self.loading == other.loading
    }
}

#[hook]
pub fn use_mutation<A, T, F, Fut>(mutation_fn: F) -> Mutation<A>
where
    A: 'static,
    T: 'static,
    F: Fn(A) -> Fut + 'static,
    Fut: std::future::Future<Output = Result<T, String>> + 'static,
{
    let loading = use_state(|| false);

    let mutate = {
        let loading = loading.clone();
        Callback::from(move |args: A| {
            let loading = loading.clone();
            loading.set(true);
            let fut = mutation_fn(args);
            wasm_bindgen_futures::spawn_local(async move {
                let _ = fut.await;
                loading.set(false);
            });
        })
    };

    Mutation {
        mutate,
        loading: *loading,
    }
}
