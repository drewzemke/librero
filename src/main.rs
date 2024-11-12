#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    use leptos::logging::log;
    use leptos::prelude::*;
    use librero::router::router;
    use sqlx::postgres::PgPoolOptions;

    // get settings from Cargo.toml
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // db setup
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Could not find db url in env.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // app setup
    let app = router(pool, leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
