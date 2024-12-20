use crate::model::{book::Book, open_library::OpenLibrarySearchResult};
use leptos::{either::Either, prelude::*};
use leptos_use::signal_debounced;
use reqwest::Url;

#[server(AddBook)]
async fn add_book(book: Book) -> Result<(), ServerFnError> {
    use sqlx::PgPool;

    let pool = expect_context::<PgPool>();

    sqlx::query_as!(
        TodoV2,
        r#"
            INSERT 
            INTO books (isbn, title, author_name, author_key) 
            VALUES ($1, $2, $3, $4)            
        "#,
        book.isbn,
        book.title,
        book.author_name,
        book.author_key
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::<sqlx::Error>::ServerError(e.to_string()))?;

    Ok(())
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
pub fn BookSearch(#[prop(into)] on_add: Callback<Book>) -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let debounced_search: Signal<String> = signal_debounced(search, 500.0);

    let results = LocalResource::new(move || search_books(debounced_search.get()));

    let handle_book_select = move |book: Book| {
        on_add.run(book);
        set_search(String::new());
    };

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
                                .into_iter()
                                .map(move |book| {
                                    view! { <BookSearchItem book=book on_select=Callback::new(handle_book_select) /> }
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
        <input
            type="search"
            bind:value=(search, set_search)
            placeholder="Search by title, author, ISBN"
        />
        <Show when=move || !debounced_search().is_empty()>
            <Suspense fallback=move || {
                view! { <p>"Searching..."</p> }
            }>
                <ul>{book_list}</ul>
            </Suspense>
        </Show>
    }
}

#[component]
fn BookSearchItem(book: Book, #[prop(into)] on_select: Callback<Book>) -> impl IntoView {
    let title = book.title.clone();
    let author = book.author_name.clone();
    let isbn = book.isbn.clone();
    view! {
        <li
            class="py-2 text-sm flex flex-col items-center max-w-36"
            aria_label=move || title.clone()
        >
            <img
                class="mb-2 aspect-square h-52 border rounded-md border-stone-600 dark:border-slate-300"
                src=move || {
                    format!("https://covers.openlibrary.org/b/isbn/{}-M.jpg", isbn.clone())
                }
            />
            <span class="text-center">{title.clone()}</span>
            <span class="text-center text-stone-600 dark:text-slate-400">{author.clone()}</span>
            <button on:click=move |_| on_select.run(book.clone())>Add</button>
        </li>
    }
}
