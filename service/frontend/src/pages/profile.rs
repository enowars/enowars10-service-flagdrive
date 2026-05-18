use leptos::prelude::*;
use crate::components::navbar::Navbar;
use crate::models::User;

#[component]
pub fn Profile(username: String) -> impl IntoView {
    let is_me = username == "me" || username == "citizen_492";
    
    let display_name = if is_me { "citizen_492".to_string() } else { username };
    
    let user = User {
        username: display_name,
        followers_count: 15,
        following_count: 8,
        is_followed: false,
    };

    view! {
        <div class="flex flex-col min-h-screen">
            <Navbar />
            
            <main class="flex-1 max-w-4xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="bg-white dark:bg-gov-surface-dark rounded-2xl border border-neutral-200 dark:border-neutral-700 relative overflow-hidden shadow-sm">
                    <div class="absolute top-0 left-0 w-full h-32 bg-neutral-100 dark:bg-gov-bg-dark border-b border-neutral-200 dark:border-neutral-700"></div>
                    
                    <div class="relative z-10 p-8">
                        <div class="flex flex-col md:flex-row items-center md:items-end gap-6 mt-4">
                            <div class="w-32 h-32 rounded-full bg-white dark:bg-gov-surface-dark border-4 border-white dark:border-neutral-800 flex items-center justify-center text-4xl font-bold text-neutral-400 shadow-md relative overflow-hidden">
                                <span class="material-icons text-6xl">"person"</span>
                                <div class="absolute bottom-2 right-2 w-4 h-4 bg-green-500 rounded-full border-2 border-white dark:border-neutral-800"></div>
                            </div>
                            
                            <div class="flex-1 text-center md:text-left mb-2">
                                <h1 class="text-3xl font-bold text-neutral-900 dark:text-white flex items-center justify-center md:justify-start">
                                    {user.username.clone()}
                                    <span class="material-icons text-orange-500 text-xl ml-2" title="Verified Citizen">"verified"</span>
                                </h1>
                                <p class="text-neutral-500 dark:text-neutral-400 text-sm mt-1">"Registered Federal Citizen"</p>
                            </div>
                            
                            <div class="flex gap-4 mb-2">
                                {if !is_me {
                                    view! {
                                        <button class="flex items-center px-6 py-2 rounded-lg font-bold text-sm bg-gov-red text-white hover:bg-gov-red-dark shadow-sm transition-all">
                                            <span class="material-icons text-[18px] mr-1">"person_add"</span>
                                            "Add to Network"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! {
                                        <button class="flex items-center px-6 py-2 rounded-lg font-bold text-sm bg-white dark:bg-gov-surface-dark border border-neutral-300 dark:border-neutral-600 text-neutral-700 dark:text-neutral-200 hover:bg-neutral-50 dark:hover:bg-neutral-700 shadow-sm transition-all">
                                            <span class="material-icons text-[18px] mr-1">"edit"</span>
                                            "Edit Profile"
                                        </button>
                                    }.into_any()
                                }}
                            </div>
                        </div>
                        
                        <div class="grid grid-cols-2 gap-4 mt-8 pt-8 border-t border-neutral-100 dark:border-neutral-700">
                            <div class="text-center p-4 bg-neutral-50 dark:bg-gov-bg-dark/50 rounded-xl border border-neutral-100 dark:border-neutral-800">
                                <div class="text-3xl font-bold text-neutral-900 dark:text-white">{user.followers_count}</div>
                                <div class="text-sm text-neutral-500 dark:text-neutral-400 mt-1 uppercase tracking-wider font-semibold">"Network Connections"</div>
                            </div>
                            <div class="text-center p-4 bg-neutral-50 dark:bg-gov-bg-dark/50 rounded-xl border border-neutral-100 dark:border-neutral-800">
                                <div class="text-3xl font-bold text-neutral-900 dark:text-white">{user.following_count}</div>
                                <div class="text-sm text-neutral-500 dark:text-neutral-400 mt-1 uppercase tracking-wider font-semibold">"Following Departments"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
