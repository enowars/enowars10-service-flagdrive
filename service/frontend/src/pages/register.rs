use leptos::prelude::*;
use crate::app::Page;

#[component]
pub fn Register() -> impl IntoView {
    let set_page = expect_context::<WriteSignal<Page>>();

    view! {
        <div class="flex-1 flex items-center justify-center min-h-screen relative overflow-hidden p-4">
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] bg-[#ff00ff]/10 rounded-full blur-[100px] pointer-events-none"></div>
            
            <div class="w-full max-w-md relative z-10 backdrop-blur-xl bg-[#131b2c]/80 border border-gray-800 rounded-2xl p-8 shadow-2xl">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-white mb-2 tracking-tight">"New Operative"</h2>
                    <p class="text-gray-400">"Register for GovNet clearance."</p>
                </div>
                
                <form class="space-y-6" on:submit=move |ev| {
                    ev.prevent_default();
                    set_page.set(Page::Login);
                }>
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">"Requested Codename"</label>
                        <input 
                            type="text" 
                            class="w-full px-4 py-3 bg-gray-900/50 border border-gray-700 rounded-lg focus:outline-none focus:border-[#ff00ff] focus:ring-1 focus:ring-[#ff00ff] text-white transition-all"
                            placeholder="agent_smith"
                            required
                        />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">"Passphrase"</label>
                        <input 
                            type="password" 
                            class="w-full px-4 py-3 bg-gray-900/50 border border-gray-700 rounded-lg focus:outline-none focus:border-[#ff00ff] focus:ring-1 focus:ring-[#ff00ff] text-white transition-all"
                            placeholder="••••••••"
                            required
                        />
                    </div>
                    
                    <button 
                        type="submit"
                        class="w-full py-3 px-4 border-2 border-[#ff00ff] text-[#ff00ff] font-bold rounded-lg hover:bg-[#ff00ff] hover:text-black hover:shadow-[0_0_20px_rgba(255,0,255,0.4)] transition-all hover:-translate-y-0.5"
                    >
                        "Submit Request"
                    </button>
                </form>
                
                <div class="mt-6 text-center">
                    <p class="text-sm text-gray-400">
                        "Already cleared? " 
                        <span 
                            class="text-[#ff00ff] hover:text-[#00f0ff] cursor-pointer transition-colors font-semibold"
                            on:click=move |_| set_page.set(Page::Login)
                        >
                            "Authenticate Here"
                        </span>
                    </p>
                </div>
            </div>
        </div>
    }
}
