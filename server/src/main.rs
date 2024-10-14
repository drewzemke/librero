use axum::{routing::get, Router};
use librero_server::libros::get_libros;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let assets_dir = ServeDir::new("assets");
    let app = Router::new()
        .nest_service("/", assets_dir)
        .route("/api/libros", get(get_libros))
        .route("/api", get(|| async { "Heyyy look an API!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("Starting server on port 4000 :)");
    axum::serve(listener, app).await.unwrap();
}
