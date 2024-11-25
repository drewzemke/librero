use crate::{
    app::{book_list::BookList, section_card::SectionCard},
    model::book::Book,
};
use leptos::prelude::*;

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

    view! {
        <SectionCard title="Recent Additions">
            <BookList title="Recent Additions" resource=recent_books />
        </SectionCard>
    }
}
