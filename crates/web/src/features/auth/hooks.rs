use yew::prelude::*;

use super::api;
use super::types::MeResponse;
use crate::hooks::{use_mutation, Mutation, QueryState};

#[hook]
pub fn use_me() -> QueryState<MeResponse> {
    crate::hooks::use_query("me", || api::get_me())
}

#[hook]
pub fn use_login() -> Mutation<(String, String)> {
    let client = crate::hooks::use_query_client();
    use_mutation(move |(email, password): (String, String)| {
        let client = client.clone();
        async move {
            let result = api::login(super::types::LoginRequest { email, password }).await;
            if result.is_ok() {
                client.invalidate("me");
            }
            result
        }
    })
}

#[hook]
pub fn use_register() -> Mutation<(String, String, String)> {
    let client = crate::hooks::use_query_client();
    use_mutation(
        move |(email, password, display_name): (String, String, String)| {
            let client = client.clone();
            async move {
                let result = api::register(super::types::RegisterRequest {
                    email,
                    password,
                    display_name,
                })
                .await;
                if result.is_ok() {
                    client.invalidate("me");
                }
                result
            }
        },
    )
}
