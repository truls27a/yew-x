use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use super::api;
use super::types::{LoginRequest, RegisterRequest};
use crate::hooks::use_query_client;
use crate::router::Route;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let email = use_state(String::new);
    let password = use_state(String::new);
    let display_name = use_state(String::new);
    let is_register = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let client = use_query_client();

    let on_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            email.set(target.value());
        })
    };

    let on_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            password.set(target.value());
        })
    };

    let on_display_name = {
        let display_name = display_name.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            display_name.set(target.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let display_name = display_name.clone();
        let is_register = is_register.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();
        let client = client.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(None);
            loading.set(true);

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let display_name_val = (*display_name).clone();
            let is_reg = *is_register;
            let navigator = navigator.clone();
            let error = error.clone();
            let loading = loading.clone();
            let client = client.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let result = if is_reg {
                    api::register(RegisterRequest {
                        email: email_val,
                        password: password_val,
                        display_name: display_name_val,
                    })
                    .await
                } else {
                    api::login(LoginRequest {
                        email: email_val,
                        password: password_val,
                    })
                    .await
                };

                loading.set(false);
                match result {
                    Ok(_) => {
                        client.invalidate("me");
                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
            });
        })
    };

    let toggle_mode = {
        let is_register = is_register.clone();
        Callback::from(move |_: MouseEvent| {
            is_register.set(!*is_register);
        })
    };

    html! {
        <div class="flex flex-col items-center justify-center min-h-[60vh] px-4">
            <div class="w-full max-w-sm">
                <h1 class="text-3xl font-bold text-white mb-8 text-center">
                    { if *is_register { "Create Account" } else { "Sign In" } }
                </h1>

                if let Some(err) = &*error {
                    <div class="bg-red-900 border border-red-700 text-red-200 px-4 py-2 rounded mb-4">
                        { err }
                    </div>
                }

                <form onsubmit={on_submit} class="flex flex-col gap-4">
                    if *is_register {
                        <input
                            type="text"
                            value={(*display_name).clone()}
                            oninput={on_display_name}
                            placeholder="Display Name"
                            class="bg-gray-900 border border-gray-700 text-white px-4 py-3 rounded-lg outline-none focus:border-blue-500 transition-colors"
                        />
                    }
                    <input
                        type="email"
                        value={(*email).clone()}
                        oninput={on_email}
                        placeholder="Email"
                        class="bg-gray-900 border border-gray-700 text-white px-4 py-3 rounded-lg outline-none focus:border-blue-500 transition-colors"
                        required=true
                    />
                    <input
                        type="password"
                        value={(*password).clone()}
                        oninput={on_password}
                        placeholder="Password"
                        class="bg-gray-900 border border-gray-700 text-white px-4 py-3 rounded-lg outline-none focus:border-blue-500 transition-colors"
                        required=true
                    />
                    <button
                        type="submit"
                        disabled={*loading}
                        class="bg-blue-500 hover:bg-blue-600 disabled:opacity-50 text-white font-bold py-3 rounded-full transition-colors"
                    >
                        { if *loading {
                            "Loading..."
                        } else if *is_register {
                            "Create Account"
                        } else {
                            "Sign In"
                        }}
                    </button>
                </form>

                <p class="text-gray-500 text-center mt-6">
                    { if *is_register { "Already have an account? " } else { "Don't have an account? " } }
                    <button onclick={toggle_mode} class="text-blue-500 hover:underline">
                        { if *is_register { "Sign In" } else { "Sign Up" } }
                    </button>
                </p>
            </div>
        </div>
    }
}
