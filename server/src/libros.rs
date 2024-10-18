use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Libro {
    isbn: String,
    title: String,
    author: String,
    cover_path: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddLibro {
    isbn: String,
}

#[utoipa::path(
    get,
    path = "/libros",
    responses(
        (status = 200, description = "Todos los libros", body = Vec<Libro>)
    )
)]
pub async fn get_libros(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Libro>>, (StatusCode, String)> {
    println!("get_libros");

    let libros = sqlx::query_as!(Libro, "SELECT isbn, title, author, cover_path FROM libros")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(libros))
}

#[utoipa::path(
    post,
    path = "/libros",
    request_body = AddLibro,
    responses(
        (status = 201, description = "Libro successfully added", body = Libro),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn add_libro(
    State(pool): State<PgPool>,
    Json(libro): Json<AddLibro>,
) -> Result<Json<Libro>, (StatusCode, String)> {
    println!("add_libro");

    let libro = sqlx::query_as!(
        Libro,
        r#"
        INSERT INTO libros (isbn, title, author)
        VALUES ($1, $2, $3)
        RETURNING isbn, title, author, cover_path
        "#,
        libro.isbn,
        "New Book!",
        "Author!!",
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(libro))
}
