use leptos::prelude::*;

use crate::pages::{
    dashboard::Dashboard,
    landing::Landing,
    login::Login,
    profile::Profile,
    register::Register,
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

    view! {
        <div class="min-h-screen bg-[#0b0f19] text-gray-200 font-sans selection:bg-neon-blue/30 flex flex-col">
            {move || match page.get() {
                Page::Landing => view! { <Landing/> }.into_any(),
                Page::Login => view! { <Login/> }.into_any(),
                Page::Register => view! { <Register/> }.into_any(),
                Page::Dashboard => view! { <Dashboard/> }.into_any(),
                Page::Profile(username) => view! { <Profile username=username/> }.into_any(),
            }}
        </div>
    }
}
