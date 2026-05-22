use leptos::prelude::*;
use crate::app::{Page, AppState};

#[component]
pub fn Navbar() -> impl IntoView {
    let state = expect_context::<AppState>();
    let page = state.page;

    let is_logged_in = move || {
        matches!(page.get(), Page::Dashboard | Page::Profile(_))
    };

    view! {
        <nav class="sticky top-0 z-50 bg-white dark:bg-gov-bg-dark border-b border-neutral-200 dark:border-neutral-800 shadow-sm transition-colors duration-300">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center space-x-4 cursor-pointer" on:click=move |_| page.set(if is_logged_in() { Page::Dashboard } else { Page::Landing })>
                        <span class="text-2xl font-bold tracking-tight text-neutral-900 dark:text-white flex items-center">
                            <span class="text-gov-red">"Flag"</span>
                            <span>"Drive"</span>
                        </span>
                    </div>

                    <div class="flex items-center space-x-1 sm:space-x-2">
                        {move || if is_logged_in() {
                            view! {
                                <button
                                    class="px-3 sm:px-4 py-2 rounded-md text-sm font-medium text-neutral-600 dark:text-neutral-300 hover:text-gov-red dark:hover:text-gov-red hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-all"
                                    on:click=move |_| page.set(Page::Dashboard)
                                >
                                    "Dashboard"
                                </button>
                                <button
                                    class="px-3 sm:px-4 py-2 rounded-md text-sm font-medium text-neutral-600 dark:text-neutral-300 hover:text-gov-red dark:hover:text-gov-red hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-all"
                                    on:click=move |_| page.set(Page::Profile("me".to_string()))
                                >
                                    "Profile"
                                </button>
                                <div class="w-px h-6 bg-neutral-300 dark:bg-neutral-700 my-auto mx-1 sm:mx-2"></div>
                                <button
                                    class="px-4 py-2 rounded-md text-sm font-medium text-neutral-700 dark:text-neutral-200 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-all"
                                    on:click=move |_| {
                                        state.set_auth_token.set(None);
                                        state.set_username.set(None);
                                        page.set(Page::Landing);
                                    }
                                >
                                    "Logout"
                                </button>
                            }.into_any()
                        } else {
                            view! {
                                <button
                                    class="px-4 py-2 rounded-md text-sm font-medium text-neutral-700 dark:text-neutral-200 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-all"
                                    on:click=move |_| page.set(Page::Login)
                                >
                                    "Sign In"
                                </button>
                                <button
                                    class="hidden sm:inline-block px-4 py-2 rounded-md text-sm font-bold bg-gov-red text-white hover:bg-gov-red-dark shadow-sm transition-all"
                                    on:click=move |_| page.set(Page::Register)
                                >
                                    "Register"
                                </button>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </nav>
    }
}
