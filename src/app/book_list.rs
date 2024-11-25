use crate::model::book::Book;
use leptos::{either::Either, prelude::*};

#[component]
pub fn BookList(
    title: &'static str,
    #[prop(into)] resource: Resource<Result<Vec<Book>, ServerFnError>>,
) -> impl IntoView {
    let books = move || {
        Suspend::new(async move {
            resource.await.map(|books| {
                if books.is_empty() {
                    Either::Left(view! { <p>{"No books found."}</p> })
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
        <Transition fallback=|| view! { <p>"Loading..."</p> }>
            <ol aria_label=title class="flex flex-wrap gap-4 justify-center">
                {books}
            </ol>
        </Transition>
    }
}

#[component]
fn BookListItem(book: Book) -> impl IntoView {
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
