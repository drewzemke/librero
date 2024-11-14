use leptos::{either::Either, prelude::*};
use leptos_use::signal_debounced;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenLibraryBook {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenLibrarySearchResult {
    docs: Vec<OpenLibraryBook>,
}

const SEARCH_URL_BASE: &'static str = "https://openlibrary.org/search.json";

// HACK: replaced reqwest::Error with String because the former is not Clone
async fn search_books(search: String) -> Result<Vec<OpenLibraryBook>, String> {
    if search.is_empty() {
        return Ok(vec![]);
    }

    let url = Url::parse(&SEARCH_URL_BASE)
        .expect("base url should be parseable")
        .query_pairs_mut()
        .append_pair("q", &search)
        .append_pair("fields", "title")
        .append_pair("limit", "5")
        .finish()
        .clone();

    let books = reqwest::get(url)
        .await
        .map_err(|_| String::from("Something went wrong."))?
        .json::<OpenLibrarySearchResult>()
        .await
        .map_err(|_| String::from("Something went wrong."))?
        .docs;

    Ok(books)
}

#[component]
pub fn BookSearch() -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let debounced_search: Signal<String> = signal_debounced(search, 500.0);

    let results = LocalResource::new(move || search_books(debounced_search.get()));

    // FIXME: why can't the return typ be `Suspend<Result<...>>`?
    // (that won't work here for some reason)
    let book_list = move || {
        Suspend::new(async move {
            match results.await {
                Ok(books) => {
                    if books.is_empty() {
                        Either::Left(view! { <p>{"Nothing found."}</p> })
                    } else {
                        Either::Right(
                            books
                                .iter()
                                .map(move |book| {
                                    view! { <li>{book.title.clone()}</li> }
                                })
                                .collect::<Vec<_>>(),
                        )
                    }
                }
                .into_any(),
                Err(err) => view! { <p>{err}</p> }.into_any(),
            }
        })
    };

    view! {
        <input bind:value=(search, set_search) />
        <Show when=move || !debounced_search().is_empty()>
            <Suspense fallback=move || {
                view! { <p>"Searching..."</p> }
            }>
                <ul>{book_list}</ul>
            </Suspense>
        </Show>
    }
}
