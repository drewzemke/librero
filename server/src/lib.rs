use axum::{routing::get, Router};
use libros::{AddLibro, Libro, __path_add_libro, __path_get_libros, add_libro, get_libros};
use sqlx::PgPool;
use testing::test_routes;
use tower_http::services::ServeDir;
use utoipa::OpenApi;

pub mod libros;
pub mod testing;

#[derive(OpenApi)]
#[openapi(paths(get_libros, add_libro), components(schemas(Libro, AddLibro)))]
pub struct ApiDoc;

// TODO: put this elsewhere?
pub fn create_app(pool: PgPool) -> Router {
    let assets_dir = ServeDir::new("assets");
    Router::new()
        .nest_service("/", assets_dir)
        .nest_service("/testing", test_routes(pool.clone()))
        .nest_service("/api", api_routes(pool))
}

pub fn api_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/libros", get(get_libros).post(add_libro))
        .route("/", get(|| async { "Heyyy look an API!" }))
        .with_state(pool)
}
