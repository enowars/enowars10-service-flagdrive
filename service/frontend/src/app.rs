use leptos::prelude::*;
use leptos_use::{ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

use crate::pages::{
    dashboard::Dashboard, landing::Landing, login::Login, profile::Profile, register::Register,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Page {
    Landing,
    Login,
    Register,
    Dashboard,
    Profile(String), // username
}

#[component]
pub fn App() -> impl IntoView {
    // Set up the global router state
    let (page, set_page) = signal(Page::Landing);
    provide_context(set_page);
    provide_context(page);

    // Setup dark mode using leptos_use
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .attribute("class")
            .emit_auto(true)
            .initial_value(ColorMode::Dark),
    );

    let toggle_mode = move |_| {
        if mode.get() == ColorMode::Dark {
            set_mode.set(ColorMode::Light);
        } else {
            set_mode.set(ColorMode::Dark);
        }
    };

    view! {
        <div class="min-h-screen bg-gov-bg-light dark:bg-gov-bg-dark text-neutral-800 dark:text-neutral-200 font-sans flex flex-col transition-colors duration-300">
            {move || match page.get() {
                Page::Landing => view! { <Landing/> }.into_any(),
                Page::Login => view! { <Login/> }.into_any(),
                Page::Register => view! { <Register/> }.into_any(),
                Page::Dashboard => view! { <Dashboard/> }.into_any(),
                Page::Profile(username) => view! { <Profile username=username/> }.into_any(),
            }}

            // Theme Toggle FAB (Floating Action Button)
            <button
                class="fixed bottom-6 right-6 p-4 rounded-full shadow-lg bg-white dark:bg-gov-surface-dark text-gov-red hover:shadow-xl hover:scale-110 transition-all border border-neutral-200 dark:border-neutral-700 flex items-center justify-center z-50"
                on:click=toggle_mode
                title="Toggle Theme"
            >
                {move || if mode.get() == ColorMode::Dark {
                    view! { <span class="material-icons">"light_mode"</span> }
                } else {
                    view! { <span class="material-icons">"dark_mode"</span> }
                }}
            </button>
        </div>
    }
}
