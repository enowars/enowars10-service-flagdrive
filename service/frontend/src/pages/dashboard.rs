use leptos::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::file_card::FileCard;
use crate::models::{File, FileVisibility};

#[component]
pub fn Dashboard() -> impl IntoView {
    let files = vec![
        File {
            id: "1".to_string(),
            name: "operation_treadstone.pdf".to_string(),
            owner: "agent_smith".to_string(),
            visibility: FileVisibility::Private,
            size: 1048576,
        },
        File {
            id: "2".to_string(),
            name: "public_disclosure_2026.txt".to_string(),
            owner: "whistleblower_99".to_string(),
            visibility: FileVisibility::Public,
            size: 2048,
        },
        File {
            id: "3".to_string(),
            name: "nexus_source_code.zip".to_string(),
            owner: "chief_architect".to_string(),
            visibility: FileVisibility::OnlyFollowed,
            size: 536870912,
        },
    ];

    view! {
        <div class="flex flex-col min-h-screen">
            <Navbar />
            
            <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="flex justify-between items-center mb-8">
                    <div>
                        <h1 class="text-3xl font-bold text-white tracking-tight">"Intelligence Dashboard"</h1>
                        <p class="text-gray-400 mt-1">"Access and distribute classified payloads."</p>
                    </div>
                    
                    <button class="flex items-center space-x-2 px-5 py-2.5 bg-gradient-to-r from-[#00f0ff] to-[#00a0ff] text-black font-semibold rounded-lg hover:shadow-[0_0_15px_rgba(0,240,255,0.4)] transition-all hover:-translate-y-0.5">
                        <span>"Upload Payload"</span>
                    </button>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {files.into_iter().map(|f| view! { <FileCard file=f /> }).collect_view()}
                </div>
            </main>
        </div>
    }
}
