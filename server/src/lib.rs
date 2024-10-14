pub mod libros;

use libros::{Libro, __path_get_libros};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_libros), components(schemas(Libro)))]
pub struct ApiDoc;
