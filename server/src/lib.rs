use axum::{routing::get, Router};
use libros::{AddLibro, Libro, __path_add_libro, __path_get_libros, add_libro, get_libros};
use sqlx::PgPool;
use tower_http::services::ServeDir;
use utoipa::OpenApi;

pub mod libros;

#[derive(OpenApi)]
#[openapi(paths(get_libros, add_libro), components(schemas(Libro, AddLibro)))]
pub struct ApiDoc;

// TODO: put this elsewhere?
pub fn create_app(pool: PgPool) -> Router {
    let assets_dir = ServeDir::new("assets");
    Router::new()
        .nest_service("/", assets_dir)
        .route("/api/libros", get(get_libros).post(add_libro))
        .route("/api", get(|| async { "Heyyy look an API!" }))
        .with_state(pool)
}
