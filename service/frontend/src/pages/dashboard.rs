use crate::app::AppState;
use crate::components::file_card::FileCard;
use crate::components::navbar::Navbar;
use flagdrive_shared::{FlagDriveFile, FlagDriveFileVisibility};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Dashboard() -> impl IntoView {
    let state = expect_context::<AppState>();
    let username = move || {
        state
            .username
            .get()
            .unwrap_or_else(|| "citizen_492".to_string())
    };

    // Upload Modal State
    let (is_dragging, set_is_dragging) = signal(false);
    let (show_modal, set_show_modal) = signal(false);
    let (visibility, set_visibility) = signal(FlagDriveFileVisibility::Private);
    let (encryption_key, set_encryption_key) = signal(String::new());
    let (selected_file, set_selected_file) = signal(None::<web_sys::File>);
    let file_input_ref = NodeRef::<leptos::html::Input>::new();

    // Download Modal State
    let (download_target, set_download_target) = signal(None::<FlagDriveFile>);
    let (decryption_key, set_decryption_key) = signal(String::new());

    let files_resource = LocalResource::new(move || {
        let user = username();
        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(&format!("http://127.0.0.1:4859/api/files/{}", user))
                .send()
                .await
                .ok()?;
            res.json::<Vec<FlagDriveFile>>().await.ok()
        }
    });

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
        if let Some(dt) = ev.data_transfer() {
            if let Some(files) = dt.files() {
                if let Some(file) = files.get(0) {
                    set_selected_file.set(Some(file));
                }
            }
        }
    };

    let on_file_change = move |ev: leptos::ev::Event| {
        if let Some(target) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            if let Some(files) = target.files() {
                if let Some(file) = files.get(0) {
                    set_selected_file.set(Some(file));
                }
            }
        }
    };

    let trigger_file_select = move |_| {
        if let Some(input) = file_input_ref.get() {
            input.click();
        }
    };

    let upload_action = Action::new_local({
        let state = state.clone();
        let files_resource = files_resource.clone();
        move |(file, enc_key, vis): &(web_sys::File, String, FlagDriveFileVisibility)| {
            let file = file.clone();
            let enc_key = enc_key.clone();
            let vis = vis.clone();
            let token = state.auth_token.get_untracked().unwrap_or_default();
            let files_resource = files_resource.clone();

            async move {
                let form_data = web_sys::FormData::new().unwrap();
                form_data
                    .append_with_blob_and_filename("file", &file, &file.name())
                    .unwrap();

                let vis_int = match vis {
                    FlagDriveFileVisibility::Public => 1,
                    FlagDriveFileVisibility::Following => 2,
                    FlagDriveFileVisibility::Followers => 3,
                    FlagDriveFileVisibility::Private => 0,
                };

                let json_payload = serde_json::json!({
                    "token": token,
                    "encryption_key": enc_key,
                    "visibility": vis_int
                });

                form_data
                    .append_with_str("json", &json_payload.to_string())
                    .unwrap();

                let mut opts = web_sys::RequestInit::new();
                opts.set_method("POST");
                opts.set_body(&form_data.into());

                let request = web_sys::Request::new_with_str_and_init(
                    "http://127.0.0.1:4859/api/file/upload",
                    &opts,
                )
                .unwrap();
                let window = web_sys::window().unwrap();

                if let Ok(resp_value) =
                    wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await
                {
                    if let Ok(resp) = resp_value.dyn_into::<web_sys::Response>() {
                        if resp.ok() {
                            files_resource.refetch();
                            return Ok(());
                        }
                    }
                }
                Err("Upload failed".to_string())
            }
        }
    });

    let download_action = Action::new_local({
        let state = state.clone();
        move |(file, dec_key): &(FlagDriveFile, String)| {
            let file = file.clone();
            let dec_key = dec_key.clone();
            let token = state.auth_token.get_untracked().unwrap_or_default();

            async move {
                let json_payload = serde_json::json!({
                    "token": token,
                    "decryption_key": dec_key
                });

                let mut opts = web_sys::RequestInit::new();
                opts.set_method("POST");
                opts.set_body(&wasm_bindgen::JsValue::from_str(&json_payload.to_string()));

                let headers = web_sys::Headers::new().unwrap();
                headers.append("Content-Type", "application/json").unwrap();
                opts.set_headers(&headers);

                let request = web_sys::Request::new_with_str_and_init(
                    &format!("http://127.0.0.1:4859/api/file/download/{}", file.id),
                    &opts,
                )
                .unwrap();
                let window = web_sys::window().unwrap();

                if let Ok(resp_value) =
                    wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await
                {
                    if let Ok(resp) = resp_value.dyn_into::<web_sys::Response>() {
                        if resp.ok() {
                            if let Ok(blob_promise) = resp.blob() {
                                if let Ok(blob_value) =
                                    wasm_bindgen_futures::JsFuture::from(blob_promise).await
                                {
                                    if let Ok(blob) = blob_value.dyn_into::<web_sys::Blob>() {
                                        let url = web_sys::Url::create_object_url_with_blob(&blob)
                                            .unwrap();
                                        let document = window.document().unwrap();
                                        let a = document
                                            .create_element("a")
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlAnchorElement>()
                                            .unwrap();
                                        a.set_href(&url);
                                        a.set_download(&file.name);
                                        a.click();
                                        web_sys::Url::revoke_object_url(&url).unwrap();
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
                Err("Download failed".to_string())
            }
        }
    });

    let trigger_download = move |file: FlagDriveFile| {
        if file.is_encrypted {
            set_download_target.set(Some(file));
            set_decryption_key.set(String::new());
        } else {
            download_action.dispatch((file, String::new()));
        }
    };

    Effect::new(move |_| {
        if let Some(Ok(())) = upload_action.value().get() {
            set_show_modal.set(false);
            set_selected_file.set(None);
            set_encryption_key.set(String::new());
            upload_action.value().set(None);
        }
        if let Some(Ok(())) = download_action.value().get() {
            set_download_target.set(None);
            set_decryption_key.set(String::new());
            download_action.value().set(None);
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
                        on:click=move |_| {
                            set_selected_file.set(None);
                            set_encryption_key.set(String::new());
                            set_show_modal.set(true);
                        }
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
                                {files.into_iter().map(move |f| {
                                    view! { <FileCard file=f on_download=trigger_download /> }
                                }).collect_view()}
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
                                    class="w-full mb-4 px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all appearance-none"
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

                                <label class="block text-sm font-bold text-neutral-700 dark:text-neutral-300 mb-2">"Encryption Key (Optional)"</label>
                                <input
                                    type="password"
                                    placeholder="Leave blank for unencrypted"
                                    class="w-full mb-6 px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all"
                                    on:input=move |ev| set_encryption_key.set(event_target_value(&ev))
                                />

                                <input
                                    type="file"
                                    class="hidden"
                                    node_ref=file_input_ref
                                    on:change=on_file_change
                                />

                                <div
                                    class="relative h-40 rounded-xl border-2 border-dashed border-neutral-300 dark:border-neutral-600 flex flex-col items-center justify-center transition-all cursor-pointer hover:bg-neutral-50 dark:hover:bg-neutral-900/30"
                                    class=("border-gov-red", move || is_dragging.get())
                                    class=("bg-red-50", move || is_dragging.get())
                                    class=("dark:bg-red-900/10", move || is_dragging.get())
                                    on:dragenter=on_drag_enter
                                    on:dragleave=on_drag_leave
                                    on:dragover=on_drag_over
                                    on:drop=on_drop
                                    on:click=trigger_file_select
                                >
                                    {move || if let Some(file) = selected_file.get() {
                                        view! {
                                            <span class="material-icons text-4xl text-green-500 mb-2">"check_circle"</span>
                                            <p class="text-neutral-900 dark:text-white font-bold px-4 truncate max-w-xs">{file.name()}</p>
                                            <p class="text-xs text-neutral-500 mt-1">"Click or drag to change file"</p>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <span class="material-icons text-4xl text-neutral-400 mb-2" class=("text-gov-red", move || is_dragging.get()) class=("animate-bounce", move || is_dragging.get())>"cloud_upload"</span>
                                            <p class="text-neutral-600 dark:text-neutral-400 font-medium text-center px-4">
                                                "Drag and drop file here, or click to browse"
                                            </p>
                                        }.into_any()
                                    }}
                                </div>
                                {move || if upload_action.pending().get() {
                                    view! { <p class="text-center text-sm text-neutral-500 mt-4">"Uploading..."</p> }.into_any()
                                } else if let Some(Err(e)) = upload_action.value().get() {
                                    view! { <p class="text-center text-sm text-red-500 mt-4 font-bold">{e}</p> }.into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}
                            </div>

                            <div class="px-6 py-4 bg-neutral-50 dark:bg-gov-bg-dark/50 border-t border-neutral-200 dark:border-neutral-700 flex justify-end gap-3">
                                <button
                                    class="px-4 py-2 font-bold text-neutral-600 dark:text-neutral-300 hover:bg-neutral-200 dark:hover:bg-neutral-700 rounded-lg transition-colors"
                                    on:click=move |_| set_show_modal.set(false)
                                >
                                    "Cancel"
                                </button>
                                <button
                                    class="px-4 py-2 font-bold text-white bg-gov-red rounded-lg shadow-sm transition-colors"
                                    class=("opacity-50", move || selected_file.get().is_none() || upload_action.pending().get())
                                    class=("cursor-not-allowed", move || selected_file.get().is_none() || upload_action.pending().get())
                                    class=("hover:bg-gov-red-dark", move || selected_file.get().is_some() && !upload_action.pending().get())
                                    disabled=move || selected_file.get().is_none() || upload_action.pending().get()
                                    on:click=move |_| {
                                        if let Some(f) = selected_file.get() {
                                            upload_action.dispatch((f, encryption_key.get(), visibility.get()));
                                        }
                                    }
                                >
                                    "Upload"
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}

            // Download Modal
            {move || if let Some(file) = download_target.get() {
                view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/60 backdrop-blur-sm">
                        <div class="bg-white dark:bg-gov-surface-dark rounded-2xl shadow-2xl w-full max-w-sm border border-neutral-200 dark:border-neutral-700 overflow-hidden flex flex-col">
                            <div class="px-6 py-4 border-b border-neutral-200 dark:border-neutral-700 flex justify-between items-center bg-neutral-50 dark:bg-gov-bg-dark/50">
                                <h3 class="text-lg font-bold text-neutral-900 dark:text-white flex items-center">
                                    <span class="material-icons mr-2 text-gov-red">"lock"</span>
                                    "Encrypted File"
                                </h3>
                                <button
                                    class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-200 transition-colors"
                                    on:click=move |_| set_download_target.set(None)
                                >
                                    <span class="material-icons">"close"</span>
                                </button>
                            </div>

                            <div class="p-6 text-center">
                                <p class="text-sm text-neutral-600 dark:text-neutral-400 mb-4">
                                    "The file " <span class="font-bold text-neutral-900 dark:text-white">{file.name.clone()}</span> " is end-to-end encrypted. Enter the decryption key to access it."
                                </p>

                                <input
                                    type="password"
                                    placeholder="Decryption Key"
                                    class="w-full mb-4 px-4 py-3 bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-300 dark:border-neutral-600 rounded-lg focus:outline-none focus:border-gov-red focus:ring-1 focus:ring-gov-red text-neutral-900 dark:text-white transition-all text-center"
                                    on:input=move |ev| set_decryption_key.set(event_target_value(&ev))
                                />
                                {move || if download_action.pending().get() {
                                    view! { <p class="text-center text-sm text-neutral-500 mt-2 mb-2">"Decrypting & Downloading..."</p> }.into_any()
                                } else if let Some(Err(e)) = download_action.value().get() {
                                    view! { <p class="text-center text-sm text-red-500 mt-2 mb-2 font-bold">{e}</p> }.into_any()
                                } else {
                                    view! { <span/> }.into_any()
                                }}
                            </div>

                            <div class="px-6 py-4 bg-neutral-50 dark:bg-gov-bg-dark/50 border-t border-neutral-200 dark:border-neutral-700 flex justify-end gap-3">
                                <button
                                    class="px-4 py-2 font-bold text-neutral-600 dark:text-neutral-300 hover:bg-neutral-200 dark:hover:bg-neutral-700 rounded-lg transition-colors w-1/2"
                                    on:click=move |_| set_download_target.set(None)
                                >
                                    "Cancel"
                                </button>
                                <button
                                    class="px-4 py-2 font-bold text-white bg-gov-red hover:bg-gov-red-dark rounded-lg shadow-sm transition-colors w-1/2 flex items-center justify-center"
                                    class=("opacity-50", move || decryption_key.get().is_empty() || download_action.pending().get())
                                    class=("cursor-not-allowed", move || decryption_key.get().is_empty() || download_action.pending().get())
                                    disabled=move || decryption_key.get().is_empty() || download_action.pending().get()
                                    on:click=move |_| {
                                        download_action.dispatch((file.clone(), decryption_key.get()));
                                    }
                                >
                                    <span class="material-icons mr-2 text-[18px]">"download"</span>
                                    "Unlock"
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
