use crate::model::book::Book;
use leptos::{either::Either, prelude::*};

use super::home::AddBook;

#[server(GetRecentBooks)]
async fn get_recent_books() -> Result<Vec<Book>, ServerFnError> {
    use sqlx::PgPool;
    let pool = expect_context::<PgPool>();

    let books = sqlx::query_as!(
        Book,
        r#"
            SELECT isbn, title, author_name, author_key 
            FROM books
            ORDER BY created_at DESC
            LIMIT 5
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::<sqlx::Error>::ServerError(e.to_string()))?;

    Ok(books)
}

#[component]
pub fn RecentAdditions(#[prop(into)] add_book: ServerAction<AddBook>) -> impl IntoView {
    let recent_books = Resource::new(move || add_book.version().get(), |_| get_recent_books());

    let books = move || {
        Suspend::new(async move {
            recent_books.await.map(|books| {
                if books.is_empty() {
                    Either::Left(view! { <p>{"You don't have any books. Add one!"}</p> })
                } else {
                    Either::Right(
                        books
                            .iter()
                            .map(move |book| {
                                let title = book.title.clone();
                                view! { <li aria_label=move || title.clone()>{title.clone()}</li> }
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
        })
    };

    view! {
        <h2>Recent Additions</h2>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ol aria_label="Recent Additions">{books}</ol>
        </Transition>
    }
}
