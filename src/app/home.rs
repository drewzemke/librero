use leptos::{logging, prelude::*, task::spawn_local};
use leptos_use::signal_debounced;
use serde::{Deserialize, Serialize};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-medium">"Librero"</h1>
        <BookSearch />
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenLibraryBook {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenLibrarySearchResult {
    docs: Vec<OpenLibraryBook>,
}

const SEARCH_URL_BASE: &'static str = "https://openlibrary.org/search.json?q=";

// TODO: put this back in the other function
async fn search_helper(url: String) -> OpenLibrarySearchResult {
    reqwest::get(url)
        .await
        .unwrap()
        .json::<OpenLibrarySearchResult>()
        .await
        .unwrap()
}

// TODO: should return errors
async fn search_books(search: String) -> Vec<String> {
    let url = {
        // TODO: better way to do this?
        let encoded_search = search.replace(' ', "+");
        logging::log!("Searching for: {encoded_search}");
        format!("{SEARCH_URL_BASE}{encoded_search}&limit=5")
    };

    let books = search_helper(url).await;

    let titles = books.docs.into_iter().map(|book| book.title).collect();

    // logging::log!("Result: {titles:?}");

    titles
}

#[component]
pub fn BookSearch() -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let debounced_search: Signal<String> = signal_debounced(search, 500.0);

    let (results, set_results) = signal(Vec::new());

    Effect::new(move |_| {
        let term = debounced_search.get();
        if term.is_empty() {
            set_results(Vec::new());
        } else {
            spawn_local(async move {
                let results = search_books(term).await;
                set_results(results);
            })
        }
    });

    view! {
        <input bind:value=(search, set_search) />
        <ul>
            <For each=results key=|book_title| book_title.clone() let:book_title>
                <li>{book_title}</li>
            </For>
        </ul>
    }
}
