use crate::app::Page;
use leptos::prelude::*;
use leptos::serde_json::json;

#[component]
pub fn Login() -> impl IntoView {
    let set_page = expect_context::<WriteSignal<Page>>();
    let set_auth_token = expect_context::<WriteSignal<Option<String>>>();

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let username = username.get();
        let password = password.get();
        
        leptos::task::spawn_local(async move {
            let client = reqwest::Client::new();
            let res = client.post("http://127.0.0.1:4859/api/auth/login")
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
                            if let Some(token) = json.get("token").and_then(|t| t.as_str()) {
                                set_auth_token.set(Some(token.to_string()));
                                set_page.set(Page::Dashboard);
                            } else {
                                set_error_msg.set(Some("Invalid response format".to_string()));
                            }
                        } else {
                            set_error_msg.set(Some("Failed to parse response".to_string()));
                        }
                    } else {
                        set_error_msg.set(Some("Login failed".to_string()));
                    }
                },
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    view! {
        <div class="flex-1 flex items-center justify-center min-h-screen p-4 bg-gov-bg-light dark:bg-gov-bg-dark">
            <div class="w-full max-w-md bg-white dark:bg-gov-surface-dark border border-neutral-200 dark:border-neutral-700 rounded-2xl p-8 shadow-lg">
                <div class="text-center mb-8 flex flex-col items-center">
                    <h2 class="text-3xl font-bold text-neutral-900 dark:text-white mb-2 tracking-tight">"Sign In"</h2>
                    <p class="text-neutral-500 dark:text-neutral-400">"Sign in to your official FlagDrive account."</p>
                </div>

                <form class="space-y-6" on:submit=on_submit>
                    <div>
                        <label class="block text-sm font-bold text-neutral-700 dark:text-neutral-300 mb-2">"Username"</label>
                        <input
                            type="text"
                            class="w-full px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all"
                            placeholder="username"
                            required
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                        />
                    </div>

                    <div>
                        <label class="block text-sm font-bold text-neutral-700 dark:text-neutral-300 mb-2">"Password"</label>
                        <input
                            type="password"
                            class="w-full px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all"
                            placeholder="••••••••"
                            required
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    {move || error_msg.get().map(|msg| view! {
                        <div class="text-red-500 text-sm font-bold">{msg}</div>
                    })}

                    <button
                        type="submit"
                        class="w-full py-3 px-4 bg-gov-red text-white font-bold rounded-lg shadow-sm hover:shadow-md hover:bg-gov-red-dark transition-all"
                    >
                        "Sign In"
                    </button>
                </form>

                <div class="mt-8 text-center pt-6 border-t border-neutral-100 dark:border-neutral-700">
                    <p class="text-sm text-neutral-600 dark:text-neutral-400">
                        "Don't have a FlagDrive account? "
                        <span
                            class="text-gov-red hover:underline cursor-pointer font-bold transition-colors"
                            on:click=move |_| set_page.set(Page::Register)
                        >
                            "Register here"
                        </span>
                    </p>
                </div>
            </div>
        </div>
    }
}
