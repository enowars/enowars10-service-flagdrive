use crate::components::navbar::Navbar;
use crate::app::{AppState, Page};
use shared::FlagDriveUser;
use leptos::prelude::*;
use leptos::serde_json;
use leptos::web_sys;

#[derive(Clone, PartialEq)]
enum ProfileModal {
    None,
    Followers,
    Following,
}

#[component]
pub fn Profile(username: String) -> impl IntoView {
    let state = expect_context::<AppState>();

    let is_me = username == "me" || Some(username.clone()) == state.username.get();

    let display_name = if is_me {
        state.username.get().unwrap_or_else(|| "citizen_492".to_string())
    } else {
        username.clone()
    };

    let user_resource = LocalResource::new({
        let display_name = display_name.clone();
        move || {
            let name = display_name.clone();
            async move {
                let client = reqwest::Client::new();
                let res = client.get(&format!("http://127.0.0.1:4859/api/user/{}", name)).send().await.ok()?;
                res.json::<FlagDriveUser>().await.ok()
            }
        }
    });

    let search_query = RwSignal::new(String::new());
    let handle_search = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let query = search_query.get();
        if !query.is_empty() {
            state.page.set(Page::Profile(query));
        }
    };

    let gdpr_action = Action::new_local(move |_: &()| {
        let token = state.auth_token.get_untracked().unwrap_or_default();
        let current_username = state.username.get_untracked().unwrap_or_default();
        async move {
            let client = reqwest::Client::new();
            let res = client.post("http://127.0.0.1:4859/api/gdpr/request")
                .json(&serde_json::json!({
                    "username": current_username,
                    "token": token
                }))
                .send()
                .await;
            
            if let Ok(resp) = res {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(gdpr_id) = json.get("gdpr_id").and_then(|id| id.as_str()) {
                        if let Some(window) = web_sys::window() {
                            let _ = window.location().assign(&format!("http://127.0.0.1:4859/api/gdpr/download/{}", gdpr_id));
                        }
                        return Ok(());
                    }
                }
            }
            Err("Failed to request GDPR data".to_string())
        }
    });

    let modal_state = RwSignal::new(ProfileModal::None);

    let modal_users_resource = LocalResource::new({
        let display_name = display_name.clone();
        move || {
            let current_modal = modal_state.get();
            let name = display_name.clone();
            async move {
                if current_modal == ProfileModal::None { return None; }
                let endpoint = if current_modal == ProfileModal::Followers { "followers" } else { "following" };
                let client = reqwest::Client::new();
                let res = client.get(&format!("http://127.0.0.1:4859/api/user/{}/{}", name, endpoint)).send().await.ok()?;
                let json = res.json::<serde_json::Value>().await.ok()?;
                
                let key = if current_modal == ProfileModal::Followers { "followers" } else { "following" };
                if let Some(users) = json.get(key).and_then(|u| u.as_array()) {
                    Some(users.iter().filter_map(|u| u.as_str().map(|s| s.to_string())).collect::<Vec<String>>())
                } else {
                    None
                }
            }
        }
    });

    view! {
        <div class="flex flex-col min-h-screen">
            <Navbar />

            <main class="flex-1 max-w-4xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-12 relative">
                
                <form on:submit=handle_search class="mb-8 flex gap-2 w-full">
                    <div class="relative flex-1">
                        <span class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <span class="material-icons text-neutral-400">"search"</span>
                        </span>
                        <input 
                            type="text" 
                            class="block w-full pl-10 pr-3 py-2 border border-neutral-300 dark:border-neutral-700 rounded-lg leading-5 bg-white dark:bg-gov-surface-dark text-neutral-900 dark:text-neutral-100 placeholder-neutral-500 focus:outline-none focus:ring-1 focus:ring-gov-red focus:border-gov-red sm:text-sm transition-colors"
                            placeholder="Search user..."
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                            prop:value=search_query
                        />
                    </div>
                    <button type="submit" class="px-4 py-2 bg-gov-red text-white font-medium rounded-lg hover:bg-gov-red-dark transition-colors shadow-sm text-sm flex items-center">
                        "Search"
                    </button>
                </form>

                <div class="bg-white dark:bg-gov-surface-dark rounded-2xl border border-neutral-200 dark:border-neutral-700 relative overflow-hidden shadow-sm">
                    <div class="absolute top-0 left-0 w-full h-32 bg-neutral-100 dark:bg-gov-bg-dark border-b border-neutral-200 dark:border-neutral-700"></div>

                    <Suspense fallback=move || view! { <div class="relative z-10 p-8 text-center">"Loading..."</div> }>
                        {
                            move || match user_resource.get() {
                            None => view! { <div></div> }.into_any(),
                            Some(None) => view! { <div class="relative z-10 p-8 text-center text-red-500">"Failed to load profile"</div> }.into_any(),
                            Some(Some(user)) => {
                                let followers_count = user.followers_count;
                                let following_count = user.following_count;
                                let u_name = user.username.clone();

                                view! {
                                    <div class="relative z-10 p-8">
                                        <div class="flex flex-col md:flex-row items-center md:items-end gap-8 mt-4 mb-6">
                                            <div class="w-40 h-40 rounded-full bg-white dark:bg-gov-surface-dark border-4 border-white dark:border-neutral-800 flex items-center justify-center text-neutral-400 shadow-md relative overflow-hidden shrink-0">
                                                <span class="material-icons mt-8" style="font-size: 200px;">"person"</span>
                                            </div>

                                            <div class="flex-1 text-center md:text-left mb-2">
                                                <h1 class="text-4xl font-bold text-neutral-900 dark:text-white">
                                                    {u_name.clone()}
                                                </h1>
                                            </div>

                                            <div class="flex justify-center md:justify-end gap-4 mb-2">
                                                {if is_me {
                                                    view! {
                                                        <button 
                                                            class="flex items-center px-4 py-2 rounded-lg font-bold text-sm bg-neutral-800 dark:bg-neutral-200 text-white dark:text-neutral-900 hover:bg-neutral-900 dark:hover:bg-white shadow-sm transition-all"
                                                            on:click=move |_| { gdpr_action.dispatch(()); }
                                                            title="Download GDPR Data"
                                                        >
                                                            <span class="material-icons text-[18px] mr-2">"download"</span>
                                                            "GDPR Data"
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <button class="flex items-center px-6 py-2 rounded-lg font-bold text-sm bg-gov-red text-white hover:bg-gov-red-dark shadow-sm transition-all">
                                                            <span class="material-icons text-[18px] mr-1">"person_add"</span>
                                                            "Add to Network"
                                                        </button>
                                                    }.into_any()
                                                }}
                                            </div>
                                        </div>

                                        <div class="grid grid-cols-2 gap-4 mt-8 pt-8 border-t border-neutral-100 dark:border-neutral-700">
                                            <div 
                                                class="text-center p-4 bg-neutral-50 dark:bg-gov-bg-dark/50 rounded-xl border border-neutral-100 dark:border-neutral-800 cursor-pointer hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                                                on:click=move |_| modal_state.set(ProfileModal::Followers)
                                            >
                                                <div class="text-3xl font-bold text-neutral-900 dark:text-white">{followers_count}</div>
                                                <div class="text-sm text-neutral-500 dark:text-neutral-400 mt-1 uppercase tracking-wider font-semibold">"Followers"</div>
                                            </div>
                                            <div 
                                                class="text-center p-4 bg-neutral-50 dark:bg-gov-bg-dark/50 rounded-xl border border-neutral-100 dark:border-neutral-800 cursor-pointer hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                                                on:click=move |_| modal_state.set(ProfileModal::Following)
                                            >
                                                <div class="text-3xl font-bold text-neutral-900 dark:text-white">{following_count}</div>
                                                <div class="text-sm text-neutral-500 dark:text-neutral-400 mt-1 uppercase tracking-wider font-semibold">"Following"</div>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </Suspense>
                </div>
            </main>

            {move || {
                let current_modal = modal_state.get();
                if current_modal != ProfileModal::None {
                    let title = if current_modal == ProfileModal::Followers { "Followers" } else { "Following" };
                    view! {
                        <div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-0">
                            <div class="fixed inset-0 bg-black/60 backdrop-blur-sm transition-opacity" on:click=move |_| modal_state.set(ProfileModal::None)></div>
                            
                            <div class="relative bg-white dark:bg-gov-surface-dark rounded-2xl shadow-2xl w-full max-w-md flex flex-col max-h-[80vh] border border-neutral-200 dark:border-neutral-700 transform transition-all">
                                <div class="px-6 py-4 border-b border-neutral-200 dark:border-neutral-800 flex justify-between items-center">
                                    <h2 class="text-xl font-bold text-neutral-900 dark:text-white">{title}</h2>
                                    <button 
                                        class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-200 transition-colors"
                                        on:click=move |_| modal_state.set(ProfileModal::None)
                                    >
                                        <span class="material-icons">"close"</span>
                                    </button>
                                </div>
                                
                                <div class="px-6 py-4 overflow-y-auto flex-1">
                                    <Suspense fallback=move || view! { <div class="text-center py-8 text-neutral-500">"Loading users..."</div> }>
                                        {move || match modal_users_resource.get() {
                                            None => view! { <div></div> }.into_any(),
                                            Some(None) => view! { <div class="text-center py-8 text-red-500">"Failed to load users"</div> }.into_any(),
                                            Some(Some(users)) => {
                                                if users.is_empty() {
                                                    view! { <div class="text-center py-8 text-neutral-500">"No users found."</div> }.into_any()
                                                } else {
                                                    view! {
                                                        <ul class="space-y-3">
                                                            {users.into_iter().map(|u| {
                                                                let u_clone = u.clone();
                                                                view! {
                                                                    <li class="flex items-center justify-between p-3 rounded-lg bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-100 dark:border-neutral-800 hover:border-gov-red/30 transition-colors">
                                                                        <div class="flex items-center gap-3 cursor-pointer" on:click=move |_| {
                                                                            modal_state.set(ProfileModal::None);
                                                                            state.page.set(Page::Profile(u_clone.clone()));
                                                                        }>
                                                                            <div class="w-10 h-10 rounded-full bg-neutral-200 dark:bg-neutral-700 flex items-center justify-center text-neutral-500 shrink-0">
                                                                                <span class="material-icons text-sm">"person"</span>
                                                                            </div>
                                                                            <span class="font-medium text-neutral-900 dark:text-white">{u.clone()}</span>
                                                                        </div>
                                                                        <button class="px-3 py-1.5 text-xs font-semibold rounded-md bg-neutral-200 dark:bg-neutral-800 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-300 dark:hover:bg-neutral-700 transition-colors">
                                                                            "Follow"
                                                                        </button>
                                                                    </li>
                                                                }
                                                            }).collect_view()}
                                                        </ul>
                                                    }.into_any()
                                                }
                                            }
                                        }}
                                    </Suspense>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }
            }}
        </div>
    }
}
