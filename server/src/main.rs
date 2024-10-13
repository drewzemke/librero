use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("assets");
    let app = Router::new()
        .route("/", get(|| async { "Ohhhh, hello!" }))
        .route("/api", get(|| async { "Heyyy look an API!" }))
        .nest_service("/assets", serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("Starting server on port 4000 :)");
    axum::serve(listener, app).await.unwrap();
}
