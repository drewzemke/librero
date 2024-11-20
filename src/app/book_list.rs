use crate::model::book::Book;
use leptos::prelude::*;

#[component]
pub fn BookList(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <ol aria_label=title class="flex flex-wrap gap-4 justify-center">
            {children()}
        </ol>
    }
}

#[component]
pub fn BookListItem(book: Book) -> impl IntoView {
    let title = book.title.clone();
    let author = book.author_name.clone();
    view! {
        <li
            class="py-2 text-sm flex flex-col items-center max-w-36"
            aria_label=move || title.clone()
        >
            <img
                class="mb-2 aspect-square h-52 border rounded-md border-stone-600 dark:border-slate-300"
                src=move || {
                    format!("https://covers.openlibrary.org/b/isbn/{}-M.jpg", book.isbn.clone())
                }
            />
            <span class="text-center">{title.clone()}</span>
            <span class="text-center text-stone-600 dark:text-slate-400">{author.clone()}</span>
        </li>
    }
}
