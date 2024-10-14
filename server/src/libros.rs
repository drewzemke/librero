use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Libro {
    isbn: String,
    title: String,
    author: String,
}

#[utoipa::path(
    get,
    path = "/libros",
    responses(
        (status = 200, description = "Todos los libros", body = Vec<Libro>)
    )
)]
pub async fn get_libros() -> Json<Vec<Libro>> {
    Json(vec![
        Libro {
            isbn: "fake_isbn_1".to_string(),
            title: "Fake Libro 1".to_string(),
            author: "Fake Author 1".to_string(),
        },
        Libro {
            isbn: "fake_isbn_2".to_string(),
            title: "Fake Libro 2".to_string(),
            author: "Fake Author 2".to_string(),
        },
        Libro {
            isbn: "fake_isbn_3".to_string(),
            title: "Fake Libro 3".to_string(),
            author: "Fake Author 3".to_string(),
        },
    ])
}
