use leptos::prelude::*;

#[component]
pub fn UserListItem<F>(
    user: String,
    is_self: bool,
    on_navigate: F,
) -> impl IntoView 
where
    F: Fn(String) + 'static + Send + Sync + Clone,
{
    let u_clone = user.clone();
    let on_nav_clone = on_navigate.clone();
    view! {
        <li class="flex items-center justify-between p-3 rounded-lg bg-neutral-50 dark:bg-gov-bg-dark border border-neutral-100 dark:border-neutral-800 hover:border-gov-red/30 transition-colors">
            <div class="flex items-center gap-3 cursor-pointer" on:click=move |_| on_nav_clone(u_clone.clone())>
                <div class="w-10 h-10 rounded-full bg-neutral-200 dark:bg-neutral-700 flex items-center justify-center text-neutral-500 shrink-0">
                    <span class="material-icons text-sm">"person"</span>
                </div>
                <span class="font-medium text-neutral-900 dark:text-white">{user.clone()}</span>
            </div>
            {if !is_self {
                view! {
                    <button class="px-3 py-1.5 text-xs font-semibold rounded-md bg-neutral-200 dark:bg-neutral-800 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-300 dark:hover:bg-neutral-700 transition-colors">
                        "Follow"
                    </button>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
        </li>
    }
}
