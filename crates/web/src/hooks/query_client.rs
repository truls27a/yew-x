use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use yew::prelude::*;

thread_local! {
    static CACHE: RefCell<HashMap<String, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

pub fn cache_get<T: Clone + 'static>(key: &str) -> Option<T> {
    CACHE.with(|c| c.borrow().get(key).and_then(|v| v.downcast_ref::<T>()).cloned())
}

pub fn cache_set<T: 'static>(key: &str, value: T) {
    CACHE.with(|c| {
        c.borrow_mut().insert(key.to_string(), Box::new(value));
    });
}

pub fn cache_remove(prefix: &str) {
    CACHE.with(|c| {
        c.borrow_mut().retain(|k, _| !k.starts_with(prefix));
    });
}

#[derive(Clone, PartialEq)]
pub struct QueryClient {
    generation: UseStateHandle<u64>,
}

impl QueryClient {
    pub fn invalidate(&self, prefix: &str) {
        cache_remove(prefix);
        self.generation.set(*self.generation + 1);
    }

    pub fn generation(&self) -> u64 {
        *self.generation
    }
}

#[derive(Properties, PartialEq)]
pub struct QueryClientProviderProps {
    pub children: Html,
}

#[function_component(QueryClientProvider)]
pub fn query_client_provider(props: &QueryClientProviderProps) -> Html {
    let generation = use_state(|| 0u64);
    let client = QueryClient { generation };

    html! {
        <ContextProvider<QueryClient> context={client}>
            { props.children.clone() }
        </ContextProvider<QueryClient>>
    }
}

#[hook]
pub fn use_query_client() -> QueryClient {
    use_context::<QueryClient>().expect("QueryClientProvider must be an ancestor")
}
