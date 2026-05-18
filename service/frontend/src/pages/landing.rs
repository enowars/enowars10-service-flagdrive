use leptos::prelude::*;
use crate::app::Page;

#[component]
pub fn Landing() -> impl IntoView {
    let set_page = expect_context::<WriteSignal<Page>>();

    view! {
        <div class="flex-1 flex flex-col items-center justify-center relative overflow-hidden min-h-screen">
            <div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MCIgaGVpZ2h0PSI0MCI+CgkJPGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMSIgZmlsbD0icmdiYSgyNTUsIDI1NSwgMjU1LCAwLjE1KSIvPgoJPC9zdmc+')] bg-repeat opacity-20"></div>
            <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-[#00f0ff]/20 rounded-full blur-[120px] pointer-events-none"></div>
            <div class="absolute top-1/3 left-1/4 -translate-x-1/2 -translate-y-1/2 w-[400px] h-[400px] bg-[#ff00ff]/20 rounded-full blur-[100px] pointer-events-none"></div>
            
            <div class="relative z-10 text-center max-w-4xl px-4 mt-[-10vh]">
                <div class="mb-6 inline-block">
                    <span class="px-3 py-1 text-xs font-bold tracking-widest uppercase border border-[#00f0ff]/50 text-[#00f0ff] rounded-full bg-[#00f0ff]/10">
                        "Top Secret / SI / TK"
                    </span>
                </div>
                
                <h1 class="text-5xl md:text-7xl lg:text-8xl font-extrabold tracking-tighter mb-8 bg-clip-text text-transparent bg-gradient-to-b from-white via-gray-200 to-gray-500 drop-shadow-sm">
                    "FLAG"<span class="bg-clip-text text-transparent bg-gradient-to-r from-[#00f0ff] to-[#ff00ff]">"DRIVE"</span>
                </h1>
                
                <p class="text-xl md:text-2xl text-gray-400 mb-12 max-w-2xl mx-auto font-light leading-relaxed">
                    "GovNet's premier classified file distribution system. End-to-end compartmentalization, absolute control."
                </p>
                
                <div class="flex flex-col sm:flex-row items-center justify-center gap-6">
                    <button 
                        class="w-full sm:w-auto px-8 py-4 rounded-lg font-bold text-lg bg-gradient-to-r from-[#00f0ff] to-[#ff00ff] text-black hover:shadow-[0_0_30px_rgba(0,240,255,0.4)] transition-all hover:-translate-y-1"
                        on:click=move |_| set_page.set(Page::Register)
                    >
                        "Initialize Access"
                    </button>
                    <button 
                        class="w-full sm:w-auto px-8 py-4 rounded-lg font-bold text-lg border-2 border-gray-700 hover:border-[#ff00ff] hover:text-[#ff00ff] hover:bg-[#ff00ff]/10 transition-all"
                        on:click=move |_| set_page.set(Page::Login)
                    >
                        "Authenticate"
                    </button>
                </div>
            </div>
        </div>
    }
}
