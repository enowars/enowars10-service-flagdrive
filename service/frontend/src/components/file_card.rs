use leptos::prelude::*;
use crate::models::{File, FileVisibility};

#[component]
pub fn FileCard(file: File) -> impl IntoView {
    let visibility_class = match file.visibility {
        FileVisibility::Private => "text-red-400 bg-red-400/10 border-red-400/20",
        FileVisibility::Public => "text-[#00f0ff] bg-[#00f0ff]/10 border-[#00f0ff]/20",
        FileVisibility::OnlyFollowed => "text-[#ff00ff] bg-[#ff00ff]/10 border-[#ff00ff]/20",
    };

    let visibility_label = match file.visibility {
        FileVisibility::Private => "Private",
        FileVisibility::Public => "Public",
        FileVisibility::OnlyFollowed => "Followers Only",
    };

    view! {
        <div class="group relative bg-[#131b2c] rounded-xl border border-gray-800 p-5 hover:border-[#00f0ff]/50 transition-all duration-300 hover:shadow-[0_0_20px_rgba(0,240,255,0.1)]">
            <div class="flex justify-between items-start mb-4">
                <div class="flex-1 min-w-0 pr-4">
                    <h3 class="text-lg font-medium text-gray-100 truncate group-hover:text-[#00f0ff] transition-colors">
                        {file.name.clone()}
                    </h3>
                    <p class="text-sm text-gray-400 mt-1">"Owner: " {file.owner.clone()}</p>
                </div>
                <span class=format!("px-2.5 py-1 text-xs font-semibold rounded border {}", visibility_class)>
                    {visibility_label}
                </span>
            </div>
            
            <div class="flex justify-between items-end mt-6">
                <span class="text-xs text-gray-500">{format!("{} bytes", file.size)}</span>
                <button class="px-3 py-1.5 text-sm font-medium text-[#00f0ff] bg-[#00f0ff]/10 rounded hover:bg-[#00f0ff]/20 transition-colors border border-[#00f0ff]/30">
                    "Download"
                </button>
            </div>
        </div>
    }
}
