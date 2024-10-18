use dotenv::dotenv;
use librero_server::create_app;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Could not find db url in env.");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    println!("Starting server on port 4000 :)");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
