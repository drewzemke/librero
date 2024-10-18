// #[cfg(feature = "e2e_tests")]
// #[cfg(test)]
mod e2e {
    use dotenv::dotenv;
    use librero_server::create_app;
    use sqlx::PgPool;
    use tokio::sync::oneshot;
    use uuid::Uuid;

    async fn create_test_database() -> (PgPool, String) {
        let master_db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        println!("{master_db_url}");

        let master_pool = PgPool::connect(&master_db_url).await.unwrap();

        let db_id = Uuid::new_v4().to_string().replace("-", "");
        let db_name = format!("librero_{}", db_id);
        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&master_pool)
            .await
            .unwrap();

        let test_db_url = format!("{}_{}", master_db_url, db_id);
        let test_pool = PgPool::connect(&test_db_url).await.unwrap();

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&test_pool)
            .await
            .expect("Failed to migrate the database");

        (test_pool, db_name)
    }

    async fn drop_test_database(db_name: &str) {
        let master_db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let master_pool = PgPool::connect(&master_db_url).await.unwrap();

        sqlx::query(&format!("DROP DATABASE {}", db_name))
            .execute(&master_pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn run_e2e_tests() {
        dotenv().ok();

        let (pool, db_name) = create_test_database().await;

        let (tx, rx) = oneshot::channel::<()>();
        let app = create_app(pool.clone());
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4001").await.unwrap();

        let server_handle = tokio::spawn(async move {
            println!("Starting server");
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    rx.await.ok();
                })
                .await
                .unwrap();
        });

        // Wait for the server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        println!("Starting client");
        let vite_handle = tokio::task::spawn_blocking(move || {
            std::process::Command::new("deno")
                .arg("task")
                .arg("--cwd")
                .arg("../client")
                .arg("dev")
                .env("VITE_SERVER_PORT", "4001")
                .env("VITE_CLIENT_PORT", "6001")
                .spawn()
                .expect("Failed to start Vite")
        });

        println!("Running tests");
        let test_handle = tokio::task::spawn_blocking(move || {
            let status = std::process::Command::new("deno")
                .arg("task")
                .arg("--cwd")
                .arg("../e2e")
                .arg("test")
                .env("CLIENT_PORT", "6001")
                .status()
                .expect("Failed to run Playwright test process");

            println!("tests complete");
            assert!(status.success(), "Tests failed");
        });

        // Wait for tests to complete
        test_handle.await.expect("Test task panicked");

        println!("Stopping Vite");
        let mut vite_process = vite_handle.await.expect("Vite task panicked");
        vite_process.kill().expect("Failed to kill Vite process");
        vite_process
            .wait()
            .expect("Failed to wait for Vite process to exit");

        println!("Telling server to shutdown");
        let _ = tx.send(());

        println!("Waiting for server to gracefully shutdown");
        let _ = server_handle.await;

        println!("Cleaning up db");
        pool.close().await;
        drop_test_database(&db_name).await;
    }
}
