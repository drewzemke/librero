use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            role="banner"
            class="fixed top-0 inset-x-0 h-14 flex items-center justify-between bg-brown-300 dark:bg-slate-950"
        >
            <div class="h-full flex items-center">
                <div class="h-full w-4 bg-cyan-300 dark:bg-cyan-500" />
                <div class="h-full w-4 bg-yellow-300 dark:bg-yellow-500" />
                <div class="h-full w-4 bg-pink-300 dark:bg-pink-500" />
                <h1 class="px-4 text-3xl font-medium text-stone-700 dark:text-slate-200">
                    "Librero"
                </h1>
            </div>
            <HeaderLinks />
        </header>
    }
}

const LINKS: [(&'static str, &'static str); 2] = [("/", "Home"), ("/library", "My Library")];

#[component]
fn HeaderLinks() -> impl IntoView {
    view! {
        <nav class="mr-4">
            <ul class="flex gap-4">
                {LINKS
                    .into_iter()
                    .map(|(href, name)| {
                        view! {
                            <li>
                                <A
                                    href=href
                                    attr:class="[&[aria-current=page]]:underline decoration-pink-500 hover:text-pink-500 transition duration-300"
                                >
                                    {name}
                                </A>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}
