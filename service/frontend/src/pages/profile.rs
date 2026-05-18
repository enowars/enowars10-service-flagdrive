use leptos::prelude::*;
use crate::components::navbar::Navbar;
use crate::models::User;

#[component]
pub fn Profile(username: String) -> impl IntoView {
    let is_me = username == "me" || username == "agent_smith";
    
    let display_name = if is_me { "agent_smith".to_string() } else { username };
    
    let user = User {
        username: display_name,
        followers_count: 1337,
        following_count: 42,
        is_followed: false,
    };

    view! {
        <div class="flex flex-col min-h-screen">
            <Navbar />
            
            <main class="flex-1 max-w-4xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="bg-[#131b2c] rounded-2xl border border-gray-800 p-8 relative overflow-hidden shadow-2xl">
                    <div class="absolute top-0 left-0 w-full h-32 bg-gradient-to-r from-[#00f0ff]/20 to-[#ff00ff]/20"></div>
                    
                    <div class="relative z-10 flex flex-col md:flex-row items-center md:items-end gap-6 mt-12">
                        <div class="w-32 h-32 rounded-full bg-gray-800 border-4 border-[#131b2c] flex items-center justify-center text-4xl font-bold text-gray-500 shadow-lg relative">
                            {user.username.chars().next().unwrap_or('?').to_uppercase().to_string()}
                            <div class="absolute bottom-1 right-1 w-5 h-5 bg-[#00f0ff] rounded-full border-4 border-[#131b2c]"></div>
                        </div>
                        
                        <div class="flex-1 text-center md:text-left mb-2">
                            <h1 class="text-3xl font-bold text-white">{user.username.clone()}</h1>
                            <p class="text-[#00f0ff] text-sm tracking-widest uppercase mt-1">"Clearance Level: Alpha"</p>
                        </div>
                        
                        <div class="flex gap-4 mb-2">
                            {if !is_me {
                                view! {
                                    <button class="px-6 py-2 rounded-lg font-bold text-sm bg-[#ff00ff]/10 text-[#ff00ff] border border-[#ff00ff]/30 hover:bg-[#ff00ff]/20 hover:shadow-[0_0_15px_rgba(255,0,255,0.3)] transition-all">
                                        "Follow Agent"
                                    </button>
                                }.into_any()
                            } else {
                                view! {
                                    <button class="px-6 py-2 rounded-lg font-bold text-sm border border-gray-600 text-gray-300 hover:bg-gray-800 transition-all">
                                        "Edit Dossier"
                                    </button>
                                }.into_any()
                            }}
                        </div>
                    </div>
                    
                    <div class="relative z-10 grid grid-cols-2 gap-4 mt-8 pt-8 border-t border-gray-800">
                        <div class="text-center p-4 bg-gray-900/50 rounded-xl border border-gray-800/50 hover:border-[#00f0ff]/30 transition-colors">
                            <div class="text-3xl font-bold text-white">{user.followers_count}</div>
                            <div class="text-sm text-gray-400 mt-1 uppercase tracking-wider">"Followers"</div>
                        </div>
                        <div class="text-center p-4 bg-gray-900/50 rounded-xl border border-gray-800/50 hover:border-[#ff00ff]/30 transition-colors">
                            <div class="text-3xl font-bold text-white">{user.following_count}</div>
                            <div class="text-sm text-gray-400 mt-1 uppercase tracking-wider">"Following"</div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
