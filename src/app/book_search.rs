use leptos::{either::Either, prelude::*};
use leptos_use::signal_debounced;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OpenLibrarySearchResult {
    docs: Vec<OpenLibraryBook>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
struct OpenLibraryBook {
    title: String,
    author_name: Option<Vec<String>>,
    author_key: Option<Vec<String>>,
    isbn: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Book {
    title: String,
    author_name: String,
    author_key: String,
    isbn: String,
}

impl TryFrom<OpenLibraryBook> for Book {
    type Error = String;

    fn try_from(book: OpenLibraryBook) -> Result<Self, Self::Error> {
        let extract_first_value = |list_opt: Option<Vec<_>>, error_msg: &str| {
            list_opt
                .and_then(|list| list.into_iter().next())
                .ok_or(String::from(error_msg))
        };

        let author_name = extract_first_value(book.author_name, "missing author_name")?;
        let author_key = extract_first_value(book.author_key, "missing author_key")?;
        let isbn = extract_first_value(book.isbn, "missing isbn")?;

        Ok(Self {
            title: book.title,
            author_name,
            author_key,
            isbn,
        })
    }
}

const SEARCH_URL_BASE: &'static str = "https://openlibrary.org/search.json";

// HACK: replaced reqwest::Error with String because the former is not Clone
async fn search_books(search: String) -> Result<Vec<Book>, String> {
    if search.is_empty() {
        return Ok(vec![]);
    }

    let url = Url::parse(&SEARCH_URL_BASE)
        .expect("base url should be parseable")
        .query_pairs_mut()
        .append_pair("q", &search)
        .append_pair("fields", "title,author_name,author_key,isbn")
        .append_pair("limit", "5")
        .finish()
        .clone();

    let open_lib_books = reqwest::get(url)
        .await
        .map_err(|err| format!("Something went wrong: {err:?}"))?
        .json::<OpenLibrarySearchResult>()
        .await
        .map_err(|err| format!("Something went wrong: {err:?}"))?
        .docs;

    let books = open_lib_books
        .into_iter()
        .filter_map(|book| book.try_into().ok())
        .collect();

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
                                    let isbn = book.isbn.clone();
                                    view! {
                                        <li>
                                            {book.title.clone()}" by "{book.author_name.clone()}
                                            <img src=move || {
                                                format!(
                                                    "https://covers.openlibrary.org/b/isbn/{isbn}-S.jpg",
                                                )
                                            } />
                                        </li>
                                    }
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
        <input bind:value=(search, set_search) placeholder="Search by title, author, ISBN" />
        <Show when=move || !debounced_search().is_empty()>
            <Suspense fallback=move || {
                view! { <p>"Searching..."</p> }
            }>
                <ul>{book_list}</ul>
            </Suspense>
        </Show>
    }
}
