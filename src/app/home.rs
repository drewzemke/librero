use leptos::{prelude::*, task::spawn_local};
use leptos_use::signal_debounced;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-medium">"Librero"</h1>
        <BookSearch />
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenLibraryBook {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenLibrarySearchResult {
    docs: Vec<OpenLibraryBook>,
}

const SEARCH_URL_BASE: &'static str = "https://openlibrary.org/search.json";

async fn search_books(search: String) -> Result<Vec<OpenLibraryBook>, reqwest::Error> {
    let url = Url::parse(&SEARCH_URL_BASE)
        .expect("base url should be parseable")
        .query_pairs_mut()
        .append_pair("q", &search)
        .append_pair("limit", "5")
        .finish()
        .clone();

    let books = reqwest::get(url)
        .await?
        .json::<OpenLibrarySearchResult>()
        .await?
        .docs;

    Ok(books)
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
                // TODO: error handling?
                if let Ok(results) = search_books(term).await {
                    set_results(results);
                }
            })
        }
    });

    view! {
        <input bind:value=(search, set_search) />
        <ul>
            <For each=results key=|book| book.title.clone() let:book>
                <li>{book.title}</li>
            </For>
        </ul>
    }
}
