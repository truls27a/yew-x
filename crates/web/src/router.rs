use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/profile/:id")]
    Profile { id: String },
    #[at("/tweet/:id")]
    TweetDetail { id: String },
    #[at("/notifications")]
    Notifications,
    #[not_found]
    #[at("/404")]
    NotFound,
}
