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

    let (show_add_form, set_show_add_form) = signal(false);

    let handle_add_book = move |book: Book| {
        add_book.dispatch(AddBook { book });
        set_show_add_form(false);
    };

    view! {
        <button
            on:click=move |_| set_show_add_form(true)
            aria_label="Add Book"
            class="absolute right-10 bottom-10 size-16
            flex justify-center items-center 
            rounded-full bg-yellow-500"
        >
            <span class="text-4xl text-slate-950">+</span>
        </button>

        <Show when=show_add_form>
            <BookSearch on_add=Callback::new(handle_add_book) />
        </Show>

        <SectionCard title="My Books">
            <BookList title="My Books" resource=library_books />
        </SectionCard>
    }
}
