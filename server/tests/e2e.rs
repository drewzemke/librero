// #[cfg(feature = "e2e_tests")]
// #[cfg(test)]
mod e2e {
    use librero_server::create_app;
    use sqlx::PgPool;
    use tokio::sync::oneshot;

    #[sqlx::test]
    async fn run_e2e_tests(pool: PgPool) {
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
            std::process::Command::new("pnpm")
                .arg("-C")
                .arg("../client")
                .arg("run")
                .arg("dev")
                .env("VITE_SERVER_PORT", "4001")
                .env("VITE_CLIENT_PORT", "6001")
                .spawn()
                .expect("Failed to start Vite")
        });

        println!("Running tests");
        let test_handle = tokio::task::spawn_blocking(move || {
            let status = std::process::Command::new("pnpm")
                .arg("-C")
                .arg("../e2e")
                .arg("run")
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
    }
}
