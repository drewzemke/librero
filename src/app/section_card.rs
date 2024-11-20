use leptos::prelude::*;

#[component]
pub fn SectionCard(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="m-4 px-4 py-2 border rounded-md
        border-cyan-300 bg-brown-400 dark:border-cyan-500 dark:bg-slate-950">
            <h2 class="mb-2 text-xl font-medium text-stone-700 dark:text-slate-200">{title}</h2>
            {children()}
        </section>
    }
}
