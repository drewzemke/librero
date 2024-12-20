use crate::{
    app::{book_list::BookList, section_card::SectionCard},
    model::book::Book,
};
use leptos::prelude::*;

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
            LIMIT 4
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

    view! {
        <SectionCard title="Featured Books">
            <Transition fallback=|| view! { <p>"Loading..."</p> }>
                <BookList title="Featured Books" resource=featured_books />
            </Transition>
        </SectionCard>
    }
}
