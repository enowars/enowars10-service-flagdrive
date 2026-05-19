use crate::components::navbar::Navbar;
use shared::User;
use leptos::prelude::*;

#[component]
pub fn Profile(username: String) -> impl IntoView {
    let is_me = username == "me" || username == "citizen_492";

    let display_name = if is_me {
        "citizen_492".to_string()
    } else {
        username.clone()
    };

    let user_resource = LocalResource::new(move || {
        let name = display_name.clone();
        async move {
            let client = reqwest::Client::new();
            let res = client.get(&format!("http://127.0.0.1:4859/api/user/{}", name)).send().await.ok()?;
            res.json::<User>().await.ok()
        }
    });

    view! {
        <div class="flex flex-col min-h-screen">
            <Navbar />

            <main class="flex-1 max-w-4xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="bg-white dark:bg-gov-surface-dark rounded-2xl border border-neutral-200 dark:border-neutral-700 relative overflow-hidden shadow-sm">
                    <div class="absolute top-0 left-0 w-full h-32 bg-neutral-100 dark:bg-gov-bg-dark border-b border-neutral-200 dark:border-neutral-700"></div>

                    <Suspense fallback=move || view! { <div class="relative z-10 p-8 text-center">"Loading..."</div> }>
                        {move || match user_resource.get() {
                            None => view! { <div></div> }.into_any(),
                            Some(None) => view! { <div class="relative z-10 p-8 text-center text-red-500">"Failed to load profile"</div> }.into_any(),
                            Some(Some(user)) => view! {
                                <div class="relative z-10 p-8">
                                    <div class="flex flex-col md:flex-row items-center md:items-end gap-8 mt-4 mb-6">
                                        <div class="w-40 h-40 rounded-full bg-white dark:bg-gov-surface-dark border-4 border-white dark:border-neutral-800 flex items-center justify-center text-neutral-400 shadow-md relative overflow-hidden shrink-0">
                                            <span class="material-icons mt-8" style="font-size: 200px;">"person"</span>
                                        </div>

                                        <div class="flex-1 text-center md:text-left mb-2">
                                            <h1 class="text-4xl font-bold text-neutral-900 dark:text-white">
                                                {user.username.clone()}
                                            </h1>
                                        </div>

                                        <div class="flex justify-center md:justify-end gap-4 mb-2">
                                            {if !is_me {
                                                view! {
                                                    <button class="flex items-center px-6 py-2 rounded-lg font-bold text-sm bg-gov-red text-white hover:bg-gov-red-dark shadow-sm transition-all">
                                                        <span class="material-icons text-[18px] mr-1">"person_add"</span>
                                                        "Add to Network"
                                                    </button>
                                                }.into_any()
                                            } else {
                                                view! { <div class="hidden"></div> }.into_any()
                                            }}
                                        </div>
                                    </div>

                                    <div class="grid grid-cols-2 gap-4 mt-8 pt-8 border-t border-neutral-100 dark:border-neutral-700">
                                        <div class="text-center p-4 bg-neutral-50 dark:bg-gov-bg-dark/50 rounded-xl border border-neutral-100 dark:border-neutral-800">
                                            <div class="text-3xl font-bold text-neutral-900 dark:text-white">{user.followers_count}</div>
                                            <div class="text-sm text-neutral-500 dark:text-neutral-400 mt-1 uppercase tracking-wider font-semibold">"Followers"</div>
                                        </div>
                                        <div class="text-center p-4 bg-neutral-50 dark:bg-gov-bg-dark/50 rounded-xl border border-neutral-100 dark:border-neutral-800">
                                            <div class="text-3xl font-bold text-neutral-900 dark:text-white">{user.following_count}</div>
                                            <div class="text-sm text-neutral-500 dark:text-neutral-400 mt-1 uppercase tracking-wider font-semibold">"Following"</div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </Suspense>
                </div>
            </main>
        </div>
    }
}
