use crate::app::book_search::BookSearch;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-medium">"Librero"</h1>
        <BookSearch />
    }
}
