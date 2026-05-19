use leptos::prelude::*;
use crate::models::{File, FileVisibility};

#[component]
pub fn FileCard(file: File) -> impl IntoView {
    let visibility_class = match file.visibility {
        FileVisibility::Private => "text-neutral-600 dark:text-neutral-400 bg-neutral-100 dark:bg-gov-surface-dark border-neutral-200 dark:border-neutral-700",
        FileVisibility::Public => "text-green-700 dark:text-green-400 bg-green-100 dark:bg-green-900/30 border-green-200 dark:border-green-800/50",
        FileVisibility::Following => "text-amber-700 dark:text-amber-400 bg-amber-100 dark:bg-amber-900/30 border-amber-200 dark:border-amber-800/50",
        FileVisibility::Followers => "text-sky-700 dark:text-sky-400 bg-sky-100 dark:bg-sky-900/30 border-sky-200 dark:border-sky-800/50",
    };

    let visibility_label = match file.visibility {
        FileVisibility::Private => "Private",
        FileVisibility::Public => "Public",
        FileVisibility::Following => "Following",
        FileVisibility::Followers => "Followers",
    };

    let icon = match file.visibility {
        FileVisibility::Private => "lock",
        FileVisibility::Public => "public",
        FileVisibility::Following => "person_add",
        FileVisibility::Followers => "groups",
    };

    view! {
        <div class="group bg-white dark:bg-gov-surface-dark rounded-xl border border-neutral-200 dark:border-neutral-700 p-5 hover:border-gov-red dark:hover:border-gov-red transition-all duration-300 hover:shadow-md">
            <div class="flex justify-between items-start mb-4">
                <div class="flex-1 min-w-0 pr-4">
                    <h3 class="text-lg font-bold text-neutral-900 dark:text-white truncate group-hover:text-gov-red transition-colors flex items-center">
                        <span class="material-icons mr-2 text-neutral-400 group-hover:text-gov-red">"description"</span>
                        {file.name.clone()}
                    </h3>
                    <p class="text-sm text-neutral-500 dark:text-neutral-400 mt-2">"Owner: " <span class="font-medium text-neutral-700 dark:text-neutral-300">{file.owner.clone()}</span></p>
                </div>
                <span class=format!("flex items-center px-2.5 py-1 text-xs font-semibold rounded-full border {}", visibility_class)>
                    <span class="material-icons text-[14px] mr-1">{icon}</span>
                    {visibility_label}
                </span>
            </div>
            
            <div class="flex justify-between items-end mt-6 border-t border-neutral-100 dark:border-neutral-700 pt-4">
                <span class="text-xs font-medium text-neutral-500 dark:text-neutral-400">{format!("{} bytes", file.size)}</span>
                <button class="p-2 text-sm font-bold text-gov-red bg-red-50 dark:bg-red-900/20 rounded hover:bg-red-100 dark:hover:bg-red-900/40 transition-colors border border-red-100 dark:border-red-900/50 flex items-center justify-center">
                    <span class="material-icons text-[20px]">"download"</span>
                </button>
            </div>
        </div>
    }
}
