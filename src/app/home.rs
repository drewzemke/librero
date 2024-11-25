use crate::app::{featured_books::FeaturedBooks, recent_additions::RecentAdditions};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <FeaturedBooks />
        <RecentAdditions />
    }
}
