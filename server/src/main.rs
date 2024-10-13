use std::fs;

use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
struct Libro {
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
async fn get_libros() -> Json<Vec<Libro>> {
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

#[derive(OpenApi)]
#[openapi(paths(get_libros), components(schemas(Libro)))]
struct ApiDoc;

fn generate_openapi_json() {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).unwrap();
    fs::write("../openapi.json", json).expect("Unable to write file");
    println!("OpenAPI JSON file generated: openapi.json");
}

#[tokio::main]
async fn main() {
    generate_openapi_json();

    let serve_dir = ServeDir::new("assets");
    let app = Router::new()
        .route("/", get(|| async { "Ohhhh, hello!" }))
        .route("/api/libros", get(get_libros))
        .route("/api", get(|| async { "Heyyy look an API!" }))
        .nest_service("/assets", serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("Starting server on port 4000 :)");
    axum::serve(listener, app).await.unwrap();
}
