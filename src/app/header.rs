use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            role="banner"
            class="fixed top-0 inset-x-0 h-14 flex items-center bg-brown-300 dark:bg-slate-950"
        >
            <div class="h-full w-4 bg-cyan-300 dark:bg-cyan-500" />
            <div class="h-full w-4 bg-yellow-300 dark:bg-yellow-500" />
            <div class="h-full w-4 bg-pink-300 dark:bg-pink-500" />
            <h1 class="px-4 text-3xl font-medium text-stone-700 dark:text-slate-200">"Librero"</h1>
        </header>
    }
}
