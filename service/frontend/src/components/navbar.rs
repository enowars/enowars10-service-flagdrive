use leptos::prelude::*;
use crate::app::Page;

#[component]
pub fn Navbar() -> impl IntoView {
    let set_page = expect_context::<WriteSignal<Page>>();

    view! {
        <nav class="sticky top-0 z-50 backdrop-blur-md bg-[#0b0f19]/80 border-b border-gray-800 shadow-[0_0_15px_rgba(0,240,255,0.1)]">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center space-x-4 cursor-pointer" on:click=move |_| set_page.set(Page::Landing)>
                        <span class="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-[#00f0ff] to-[#ff00ff] tracking-tight">
                            "FlagDrive"
                        </span>
                        <span class="text-xs px-2 py-0.5 rounded-full bg-red-500/20 text-red-400 border border-red-500/30 uppercase tracking-wider font-semibold">
                            "GovNet Access"
                        </span>
                    </div>
                    <div class="flex space-x-2">
                        <button 
                            class="px-4 py-2 rounded-md text-sm font-medium text-gray-400 hover:text-[#00f0ff] hover:bg-gray-800/50 transition-all"
                            on:click=move |_| set_page.set(Page::Dashboard)
                        >
                            "Dashboard"
                        </button>
                        <button 
                            class="px-4 py-2 rounded-md text-sm font-medium text-gray-400 hover:text-[#00f0ff] hover:bg-gray-800/50 transition-all"
                            on:click=move |_| set_page.set(Page::Profile("me".to_string()))
                        >
                            "Profile"
                        </button>
                        <div class="w-px h-6 bg-gray-700 my-auto mx-2"></div>
                        <button 
                            class="px-5 py-2 rounded-md text-sm font-semibold bg-[#ff00ff]/10 text-[#ff00ff] border border-[#ff00ff]/30 hover:bg-[#ff00ff]/20 hover:shadow-[0_0_10px_rgba(255,0,255,0.3)] transition-all"
                            on:click=move |_| set_page.set(Page::Login)
                        >
                            "Logout"
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}
