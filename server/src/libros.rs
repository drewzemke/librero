use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Libro {
    isbn: String,
    title: String,
    author: String,
    cover_path: String,
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
            isbn: "9780525620792".to_string(),
            title: "Mexican Gothic 1".to_string(),
            author: "Silvia Moreno-Garcia".to_string(),
            cover_path: "covers/53117768.jpg".to_string(),
        },
        Libro {
            isbn: "9780525620792".to_string(),
            title: "Mexican Gothic 2".to_string(),
            author: "Silvia Moreno-Garcia".to_string(),
            cover_path: "covers/53117768.jpg".to_string(),
        },
        Libro {
            isbn: "9780525620792".to_string(),
            title: "Mexican Gothic 3".to_string(),
            author: "Silvia Moreno-Garcia".to_string(),
            cover_path: "covers/53117768.jpg".to_string(),
        },
    ])
}
