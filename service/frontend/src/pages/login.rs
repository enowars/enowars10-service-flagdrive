use leptos::prelude::*;
use crate::app::Page;

#[component]
pub fn Login() -> impl IntoView {
    let set_page = expect_context::<WriteSignal<Page>>();

    view! {
        <div class="flex-1 flex items-center justify-center min-h-screen p-4 bg-gov-bg-light dark:bg-gov-bg-dark">
            <div class="w-full max-w-md bg-white dark:bg-gov-surface-dark border border-neutral-200 dark:border-neutral-700 rounded-2xl p-8 shadow-lg">
                <div class="text-center mb-8 flex flex-col items-center">
                    <span class="material-icons text-5xl text-gov-red mb-4">"account_balance"</span>
                    <h2 class="text-3xl font-bold text-neutral-900 dark:text-white mb-2 tracking-tight">"Citizen Portal"</h2>
                    <p class="text-neutral-500 dark:text-neutral-400">"Sign in to access your secure federal documents."</p>
                </div>
                
                <form class="space-y-6" on:submit=move |ev| {
                    ev.prevent_default();
                    set_page.set(Page::Dashboard);
                }>
                    <div>
                        <label class="block text-sm font-bold text-neutral-700 dark:text-neutral-300 mb-2">"Federal ID / Username"</label>
                        <input 
                            type="text" 
                            class="w-full px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all"
                            placeholder="e.g. citizen_492"
                            required
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-bold text-neutral-700 dark:text-neutral-300 mb-2">"Password"</label>
                        <input 
                            type="password" 
                            class="w-full px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all"
                            placeholder="••••••••"
                            required
                        />
                    </div>
                    
                    <button 
                        type="submit"
                        class="w-full py-3 px-4 bg-gov-red text-white font-bold rounded-lg shadow-sm hover:shadow-md hover:bg-gov-red-dark transition-all"
                    >
                        "Secure Sign In"
                    </button>
                </form>
                
                <div class="mt-8 text-center pt-6 border-t border-neutral-100 dark:border-neutral-700">
                    <p class="text-sm text-neutral-600 dark:text-neutral-400">
                        "Don't have a federal account? " 
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
