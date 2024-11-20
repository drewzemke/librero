use crate::{
    app::{
        book_search::BookSearch, featured_books::FeaturedBooks, recent_additions::RecentAdditions,
    },
    model::book::Book,
};
use leptos::prelude::*;

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

#[component]
pub fn HomePage() -> impl IntoView {
    let add_book = ServerAction::<AddBook>::new();

    view! {
        <FeaturedBooks />
        <BookSearch add_book=add_book />
        <RecentAdditions add_book=add_book />
    }
}
