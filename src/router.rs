use crate::app::{shell, App};
use axum::Router;
use leptos::{config::LeptosOptions, prelude::provide_context};
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::PgPool;

pub fn create_router(pool: PgPool, leptos_options: LeptosOptions) -> Router {
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(pool.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
}
