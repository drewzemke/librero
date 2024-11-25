use header::Header;
use home::HomePage;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use library::Library;

pub mod book_list;
pub mod book_search;
pub mod featured_books;
pub mod header;
pub mod home;
pub mod library;
pub mod recent_additions;
pub mod section_card;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="font-inter overflow-hidden
            bg-gradient-to-t from-brown-800 to-brown-500 text-stone-900
            dark:from-slate-950 dark:to-indigo-950 dark:text-slate-200">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/librero.css" />

        // sets the document title
        <Title text="Librero" />

        // content for this welcome page
        <Router>
            <Header />
            <main class="w-full h-screen pt-14 overflow-y-auto">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("library") view=Library />
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
