use crate::{
    app::{
        book_list::{BookList, BookListItem},
        section_card::SectionCard,
    },
    model::book::Book,
};
use leptos::{either::Either, prelude::*};

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
            LIMIT 4
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::<sqlx::Error>::ServerError(e.to_string()))?;

    Ok(books)
}

#[component]
pub fn RecentAdditions() -> impl IntoView {
    let recent_books = Resource::new(move || {}, |_| get_recent_books());

    let books = move || {
        Suspend::new(async move {
            recent_books.await.map(|books| {
                if books.is_empty() {
                    Either::Left(view! { <p>{"You don't have any books. Add one!"}</p> })
                } else {
                    Either::Right(
                        books
                            .into_iter()
                            .map(move |book| {
                                view! { <BookListItem book=book /> }
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
        })
    };

    view! {
        <SectionCard title="Recent Additions">
            <Transition fallback=|| view! { <p>"Loading..."</p> }>
                <BookList title="Recent Additions">{books}</BookList>
            </Transition>
        </SectionCard>
    }
}
