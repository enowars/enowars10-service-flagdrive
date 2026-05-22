use crate::components::navbar::Navbar;
use crate::components::profile_card::ProfileCard;
use crate::components::user_list_item::UserListItem;
use crate::app::{AppState, Page};
use shared::FlagDriveUser;
use leptos::prelude::*;
use leptos::serde_json;
use leptos::web_sys;

#[derive(Clone, PartialEq)]
pub enum ProfileModal {
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
            let logged_in_user = state.username.get();
            async move {
                let client = reqwest::Client::new();
                let res = client.get(&format!("http://127.0.0.1:4859/api/user/{}", name)).send().await.ok()?;
                let mut user = res.json::<FlagDriveUser>().await.ok()?;
                
                if let Some(me) = logged_in_user {
                    if me.to_lowercase() == name.to_lowercase() {
                        user.is_followed = false;
                    } else if let Ok(resp) = client.get(&format!("http://127.0.0.1:4859/api/user/{}/following", me)).send().await {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(following) = json.get("following").and_then(|f| f.as_array()) {
                                let is_following = following.iter().any(|val| {
                                    val.as_str().map(|s| s.to_lowercase() == name.to_lowercase()).unwrap_or(false)
                                });
                                user.is_followed = is_following;
                            }
                        }
                    }
                }
                Some(user)
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

    let follow_action = Action::new_local({
        let display_name = display_name.clone();
        move |is_following: &bool| {
            let is_following = *is_following;
            let display_name = display_name.clone();
            let token = state.auth_token.get_untracked().unwrap_or_default();
            let current_username = state.username.get_untracked().unwrap_or_default();
            async move {
                let action_endpoint = if is_following { "unfollow" } else { "follow" };
                let client = reqwest::Client::new();
                let res = client.post(&format!("http://127.0.0.1:4859/api/user/{}/{}", current_username, action_endpoint))
                    .json(&serde_json::json!({
                        "username": display_name,
                        "token": token
                    }))
                    .send()
                    .await;
                
                match res {
                    Ok(resp) if resp.status().is_success() => Ok(()),
                    _ => Err("Failed to update follow relationship".to_string()),
                }
            }
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(())) = follow_action.value().get() {
            user_resource.refetch();
        }
    });

    let modal_state = RwSignal::new(ProfileModal::None);

    let modal_users_resource = LocalResource::new({
        let display_name = display_name.clone();
        let state = state.clone();
        move || {
            let current_modal = modal_state.get();
            let name = display_name.clone();
            let logged_in_user = state.username.get();
            async move {
                if current_modal == ProfileModal::None { return None; }
                let endpoint = if current_modal == ProfileModal::Followers { "followers" } else { "following" };
                let client = reqwest::Client::new();
                
                let res = client.get(&format!("http://127.0.0.1:4859/api/user/{}/{}", name, endpoint)).send().await.ok()?;
                let json = res.json::<serde_json::Value>().await.ok()?;
                
                let key = if current_modal == ProfileModal::Followers { "followers" } else { "following" };
                let users_list = json.get(key).and_then(|u| u.as_array())?;
                let usernames: Vec<String> = users_list.iter().filter_map(|u| u.as_str().map(|s| s.to_string())).collect();
                
                let mut my_following = std::collections::HashSet::new();
                if let Some(me) = logged_in_user {
                    if let Ok(resp) = client.get(&format!("http://127.0.0.1:4859/api/user/{}/following", me)).send().await {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(following) = json.get("following").and_then(|f| f.as_array()) {
                                for val in following {
                                    if let Some(s) = val.as_str() {
                                        my_following.insert(s.to_lowercase());
                                    }
                                }
                            }
                        }
                    }
                }
                
                let result: Vec<(String, bool)> = usernames.into_iter().map(|u| {
                    let is_followed = my_following.contains(&u.to_lowercase());
                    (u, is_followed)
                }).collect();
                
                Some(result)
            }
        }
    });

    let modal_follow_action = Action::new_local({
        let state = state.clone();
        move |(target_user, is_following): &(String, bool)| {
            let is_following = *is_following;
            let target_user = target_user.clone();
            let token = state.auth_token.get_untracked().unwrap_or_default();
            let current_username = state.username.get_untracked().unwrap_or_default();
            async move {
                let action_endpoint = if is_following { "unfollow" } else { "follow" };
                let client = reqwest::Client::new();
                let res = client.post(&format!("http://127.0.0.1:4859/api/user/{}/{}", current_username, action_endpoint))
                    .json(&serde_json::json!({
                        "username": target_user,
                        "token": token
                    }))
                    .send()
                    .await;
                
                match res {
                    Ok(resp) if resp.status().is_success() => Ok(()),
                    _ => Err("Failed to update follow relationship".to_string()),
                }
            }
        }
    });

    Effect::new({
        let modal_users_resource = modal_users_resource.clone();
        let user_resource = user_resource.clone();
        move |_| {
            if let Some(Ok(())) = modal_follow_action.value().get() {
                modal_users_resource.refetch();
                user_resource.refetch();
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
                                view! {
                                    <ProfileCard 
                                        user=user 
                                        is_me=is_me 
                                        gdpr_action=gdpr_action 
                                        follow_action=follow_action
                                        modal_state=modal_state 
                                    />
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
                                                    let modal_follow_clone = modal_follow_action.clone();
                                                    view! {
                                                        <ul class="space-y-3">
                                                            {users.into_iter().map(|(u, is_followed)| {
                                                                let state_clone = state.clone();
                                                                let u_clone = u.clone();
                                                                let on_nav = move |_nav_u| {
                                                                    modal_state.set(ProfileModal::None);
                                                                    state_clone.page.set(Page::Profile(u_clone.clone()));
                                                                };
                                                                let modal_follow_dispatch = modal_follow_clone.clone();
                                                                let on_toggle = move |target_user: String, cur_following: bool| {
                                                                    modal_follow_dispatch.dispatch((target_user, cur_following));
                                                                };
                                                                view! {
                                                                    <UserListItem user=u is_followed=is_followed on_navigate=on_nav on_toggle=on_toggle />
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
