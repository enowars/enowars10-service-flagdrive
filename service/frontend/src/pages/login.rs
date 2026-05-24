use crate::app::{AppState, Page};
use crate::components::auth_form::AuthForm;
use leptos::prelude::*;
use leptos::serde_json::json;

#[component]
pub fn Login() -> impl IntoView {
    let state = expect_context::<AppState>();

    let login_action = Action::new_local(|(username, password): &(String, String)| {
        let username = username.clone();
        let password = password.clone();
        async move {
            let client = reqwest::Client::new();
            let res = client.post("/api/auth/login")
                .json(&json!({
                    "username": username,
                    "password": password
                }))
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        if let Ok(json) = resp.json::<leptos::serde_json::Value>().await {
                            let token = json.get("token").and_then(|t| t.as_str());
                            let username_val = json.get("username").and_then(|u| u.as_str());
                            if let (Some(token), Some(username_val)) = (token, username_val) {
                                Ok((token.to_string(), username_val.to_string()))
                            } else {
                                Err("Invalid response format".to_string())
                            }
                        } else {
                            Err("Failed to parse response".to_string())
                        }
                    } else {
                        if let Ok(json) = resp.json::<leptos::serde_json::Value>().await {
                            if let Some(err_msg) = json.get("error").and_then(|e| e.as_str()) {
                                Err(err_msg.to_string())
                            } else {
                                Err("Login failed".to_string())
                            }
                        } else {
                            Err("Login failed".to_string())
                        }
                    }
                }
                Err(e) => Err(e.to_string()),
            }
        }
    });

    let value = login_action.value();

    Effect::new(move |_| {
        if let Some(Ok((token, username))) = value.get() {
            state.set_auth_token.set(Some(token));
            state.set_username.set(Some(username));
            state.page.set(Page::Dashboard);
        }
    });

    let on_submit = move |username: String, password: String| {
        login_action.dispatch((username, password));
    };

    let is_pending = Signal::derive(move || login_action.pending().get());
    let error_msg = Signal::derive(move || {
        value.get().and_then(|res| res.err())
    });

    view! {
        <AuthForm
            title="Sign In"
            subtitle="Sign in to your official FlagDrive account."
            button_label="Sign In"
            loading_label="Signing In..."
            is_pending=is_pending
            error_msg=error_msg
            on_submit=on_submit
            footer=move || view! {
                <p class="text-sm text-neutral-600 dark:text-neutral-400">
                    "Don't have a FlagDrive account? "
                    <span
                        class="text-gov-red hover:underline cursor-pointer font-bold transition-colors"
                        on:click=move |_| state.page.set(Page::Register)
                    >
                        "Register here"
                    </span>
                </p>
            }
        />
    }
}
