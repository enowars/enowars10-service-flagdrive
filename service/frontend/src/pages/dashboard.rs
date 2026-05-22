use crate::components::file_card::FileCard;
use crate::components::navbar::Navbar;
use crate::app::AppState;
use shared::{FlagDriveFile, FlagDriveFileVisibility};
use leptos::prelude::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    let state = expect_context::<AppState>();
    let username = move || state.username.get().unwrap_or_else(|| "citizen_492".to_string());

    let (is_dragging, set_is_dragging) = signal(false);
    let (show_modal, set_show_modal) = signal(false);
    let (_visibility, set_visibility) = signal(FlagDriveFileVisibility::Private);

    let on_drag_enter = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(true);
    };

    let on_drag_leave = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(false);
    };

    let on_drag_over = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(true);
    };

    let on_drop = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        set_is_dragging.set(false);
        // Mock: File dropped in modal!
        set_show_modal.set(false);
    };

    let files_resource = LocalResource::new(move || {
        let user = username();
        async move {
            let client = reqwest::Client::new();
            let res = client.get(&format!("http://127.0.0.1:4859/api/files/{}", user)).send().await.ok()?;
            res.json::<Vec<FlagDriveFile>>().await.ok()
        }
    });

    view! {
        <div class="flex flex-col min-h-screen">
            <Navbar />

            <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-8 relative">

                <div class="flex justify-between items-center mb-8">
                    <div>
                        <h1 class="text-3xl font-bold text-neutral-900 dark:text-white tracking-tight flex items-center">
                            <span class="material-icons mr-2 text-gov-red text-3xl">"folder"</span>
                            "Documents"
                        </h1>
                        <p class="text-neutral-600 dark:text-neutral-400 mt-1">"Access, manage, and securely upload your documents."</p>
                    </div>

                    <button
                        class="flex items-center space-x-2 px-5 py-2.5 bg-gov-red text-white font-bold rounded-lg hover:bg-gov-red-dark shadow-sm hover:shadow-md transition-all"
                        on:click=move |_| set_show_modal.set(true)
                    >
                        <span class="material-icons">"upload_file"</span>
                        <span>"Upload File"</span>
                    </button>
                </div>

                <Suspense fallback=move || view! { <div class="text-center p-8">"Loading..."</div> }>
                    {move || match files_resource.get() {
                        None => view! { <div></div> }.into_any(),
                        Some(None) => view! { <div class="text-center p-8 text-red-500">"Failed to load files"</div> }.into_any(),
                        Some(Some(files)) => view! {
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                {files.into_iter().map(|f| view! { <FileCard file=f /> }).collect_view()}
                            </div>
                        }.into_any()
                    }}
                </Suspense>
            </main>

            // Upload Modal
            {move || if show_modal.get() {
                view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/60 backdrop-blur-sm">
                        <div class="bg-white dark:bg-gov-surface-dark rounded-2xl shadow-2xl w-full max-w-lg border border-neutral-200 dark:border-neutral-700 overflow-hidden flex flex-col">
                            <div class="px-6 py-4 border-b border-neutral-200 dark:border-neutral-700 flex justify-between items-center bg-neutral-50 dark:bg-gov-bg-dark/50">
                                <h3 class="text-lg font-bold text-neutral-900 dark:text-white flex items-center">
                                    <span class="material-icons mr-2 text-gov-red">"upload"</span>
                                    "Secure Upload"
                                </h3>
                                <button
                                    class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-200 transition-colors"
                                    on:click=move |_| set_show_modal.set(false)
                                >
                                    <span class="material-icons">"close"</span>
                                </button>
                            </div>

                            <div class="p-6">
                                <label class="block text-sm font-bold text-neutral-700 dark:text-neutral-300 mb-2">"Visibility Clearance"</label>
                                <select
                                    class="w-full mb-6 px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all appearance-none"
                                    on:change=move |ev| {
                                        let val = event_target_value(&ev);
                                        let vis = match val.as_str() {
                                            "Public" => FlagDriveFileVisibility::Public,
                                            "Following" => FlagDriveFileVisibility::Following,
                                            "Followers" => FlagDriveFileVisibility::Followers,
                                            _ => FlagDriveFileVisibility::Private,
                                        };
                                        set_visibility.set(vis);
                                    }
                                >
                                    <option value="Private" selected=true>"Private (Only Me)"</option>
                                    <option value="Following">"Following (Only people I follow)"</option>
                                    <option value="Followers">"Followers (Only my followers)"</option>
                                    <option value="Public">"Public (Everyone)"</option>
                                </select>

                                <div
                                    class="relative h-48 rounded-xl border-2 border-dashed border-neutral-300 dark:border-neutral-600 flex flex-col items-center justify-center transition-all cursor-pointer hover:bg-neutral-50 dark:hover:bg-neutral-900/30"
                                    class=("border-gov-red", move || is_dragging.get())
                                    class=("bg-red-50", move || is_dragging.get())
                                    class=("dark:bg-red-900/10", move || is_dragging.get())
                                    on:dragenter=on_drag_enter
                                    on:dragleave=on_drag_leave
                                    on:dragover=on_drag_over
                                    on:drop=on_drop
                                >
                                    <span class="material-icons text-5xl text-neutral-400 mb-3" class=("text-gov-red", move || is_dragging.get()) class=("animate-bounce", move || is_dragging.get())>"cloud_upload"</span>
                                    <p class="text-neutral-600 dark:text-neutral-400 font-medium text-center px-4">
                                        "Drag and drop file here, or click to browse"
                                    </p>
                                    <p class="text-xs text-neutral-400 dark:text-neutral-500 mt-2">"Maximum file size: 50MB"</p>
                                </div>
                            </div>

                            <div class="px-6 py-4 bg-neutral-50 dark:bg-gov-bg-dark/50 border-t border-neutral-200 dark:border-neutral-700 flex justify-end gap-3">
                                <button
                                    class="px-4 py-2 font-bold text-neutral-600 dark:text-neutral-300 hover:bg-neutral-200 dark:hover:bg-neutral-700 rounded-lg transition-colors"
                                    on:click=move |_| set_show_modal.set(false)
                                >
                                    "Cancel"
                                </button>
                                <button class="px-4 py-2 font-bold text-white bg-gov-red hover:bg-gov-red-dark rounded-lg shadow-sm transition-colors opacity-50 cursor-not-allowed">
                                    "Upload"
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
        </div>
    }
}
