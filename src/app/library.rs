use crate::{
    app::{
        book_list::BookList,
        book_search::{AddBook, BookSearch},
        section_card::SectionCard,
    },
    model::book::Book,
};
use leptos::prelude::*;

#[server(GetLibraryBooks)]
async fn get_library_books() -> Result<Vec<Book>, ServerFnError> {
    use sqlx::PgPool;
    let pool = expect_context::<PgPool>();

    let books = sqlx::query_as!(
        Book,
        r#"
            SELECT isbn, title, author_name, author_key 
            FROM books
            ORDER BY created_at DESC
            LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::<sqlx::Error>::ServerError(e.to_string()))?;

    Ok(books)
}

#[component]
pub fn Library() -> impl IntoView {
    let add_book = ServerAction::<AddBook>::new();

    let library_books = Resource::new(move || add_book.version().get(), |_| get_library_books());

    view! {
        <BookSearch add_book=add_book />
        <SectionCard title="My Books">
            <BookList title="My Books" resource=library_books />
        </SectionCard>
    }
}
