use leptos::prelude::*;
use crate::app::Page;

#[component]
pub fn Login() -> impl IntoView {
    let set_page = expect_context::<WriteSignal<Page>>();

    view! {
        <div class="flex-1 flex items-center justify-center min-h-screen relative overflow-hidden p-4">
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] bg-[#00f0ff]/10 rounded-full blur-[100px] pointer-events-none"></div>
            
            <div class="w-full max-w-md relative z-10 backdrop-blur-xl bg-[#131b2c]/80 border border-gray-800 rounded-2xl p-8 shadow-2xl">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-white mb-2 tracking-tight">"Agent Login"</h2>
                    <p class="text-gray-400">"Enter your credentials to access GovNet."</p>
                </div>
                
                <form class="space-y-6" on:submit=move |ev| {
                    ev.prevent_default();
                    set_page.set(Page::Dashboard);
                }>
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">"Codename / Username"</label>
                        <input 
                            type="text" 
                            class="w-full px-4 py-3 bg-gray-900/50 border border-gray-700 rounded-lg focus:outline-none focus:border-[#00f0ff] focus:ring-1 focus:ring-[#00f0ff] text-white transition-all"
                            placeholder="agent_smith"
                            required
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">"Passphrase"</label>
                        <input 
                            type="password" 
                            class="w-full px-4 py-3 bg-gray-900/50 border border-gray-700 rounded-lg focus:outline-none focus:border-[#00f0ff] focus:ring-1 focus:ring-[#00f0ff] text-white transition-all"
                            placeholder="••••••••"
                            required
                        />
                    </div>
                    
                    <button 
                        type="submit"
                        class="w-full py-3 px-4 bg-gradient-to-r from-[#00f0ff] to-[#ff00ff] text-black font-bold rounded-lg hover:shadow-[0_0_20px_rgba(0,240,255,0.3)] transition-all hover:-translate-y-0.5"
                    >
                        "Authenticate"
                    </button>
                </form>
                
                <div class="mt-6 text-center">
                    <p class="text-sm text-gray-400">
                        "Unregistered? " 
                        <span 
                            class="text-[#00f0ff] hover:text-[#ff00ff] cursor-pointer transition-colors font-semibold"
                            on:click=move |_| set_page.set(Page::Register)
                        >
                            "Request Access"
                        </span>
                    </p>
                </div>
            </div>
        </div>
    }
}
