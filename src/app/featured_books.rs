use crate::model::book::Book;
use leptos::{either::Either, prelude::*};

#[server(GetFeaturedBooks)]
async fn get_featured_books() -> Result<Vec<Book>, ServerFnError> {
    use sqlx::PgPool;
    let pool = expect_context::<PgPool>();

    let books = sqlx::query_as!(
        Book,
        r#"
            SELECT isbn, title, author_name, author_key 
            FROM books
            ORDER BY RANDOM()
            LIMIT 3
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::<sqlx::Error>::ServerError(e.to_string()))?;

    Ok(books)
}

#[component]
pub fn FeaturedBooks() -> impl IntoView {
    let featured_books = Resource::new(move || {}, |_| get_featured_books());

    let books = move || {
        Suspend::new(async move {
            featured_books.await.map(|books| {
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
        <h2>"Featured Books"</h2>
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ol aria_label="Featured Books">{books}</ol>
        </Transition>
    }
}
