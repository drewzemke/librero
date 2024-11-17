use crate::router::router;
use axum::{extract::State, routing::post, Json, Router};
use axum_macros::debug_handler;
use leptos::config::LeptosOptions;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::oneshot::{self, Sender},
    task::JoinHandle,
};

#[derive(Debug, Deserialize)]
struct TestRequest {
    #[serde(rename = "testId")]
    test_id: String,
}

#[derive(Debug, Serialize)]
struct TestResponse {
    #[serde(rename = "clientPort")]
    client_port: u16,
}

#[derive(Debug)]
struct TestRun {
    kill_server: Sender<()>,
    server_handle: JoinHandle<()>,
}

#[derive(Clone)]
struct TestState {
    pool: PgPool,
    leptos_options: LeptosOptions,
    runs: Arc<Mutex<HashMap<String, TestRun>>>,
}

pub fn test_routes(pool: PgPool, leptos_options: LeptosOptions) -> Router {
    let test_state = TestState {
        pool,
        leptos_options,
        runs: Arc::new(Mutex::new(HashMap::new())),
    };
    Router::new()
        .route("/start", post(setup_test))
        .route("/stop", post(teardown_test))
        .with_state(test_state)
}

// TODO: replace logs here with tracing
async fn setup_test(
    State(state): State<TestState>,
    Json(body): Json<TestRequest>,
) -> Json<TestResponse> {
    println!("Setting up test for ID: {}", body.test_id);

    // make a bespoke db for this test
    let (test_pool, db_name) = create_test_database(body.test_id.clone(), state.pool).await;
    println!("Created this db: {db_name}");

    // spin up the server
    let (tx, rx) = oneshot::channel::<()>();
    let router = router(test_pool.clone(), state.leptos_options.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let server_port = listener.local_addr().unwrap().port();

    let server_handle = tokio::spawn(async move {
        println!("Starting server on port {server_port}");
        axum::serve(listener, router)
            .with_graceful_shutdown(async {
                rx.await.ok();
            })
            .await
            .unwrap();
    });

    // store data in state
    let mut runs = state.runs.lock().unwrap();
    runs.insert(
        body.test_id,
        TestRun {
            kill_server: tx,
            server_handle,
        },
    );

    Json(TestResponse {
        client_port: server_port,
    })
}

#[debug_handler]
async fn teardown_test(
    State(state): State<TestState>,
    Json(body): Json<TestRequest>,
) -> Json<TestResponse> {
    let run_data = {
        let mut runs = state.runs.lock().unwrap();
        runs.remove(&body.test_id).expect("couldn't find test data")
    };
    println!("Tearing down test for ID: {}", body.test_id);

    println!("Telling server to shutdown");
    run_data.kill_server.send(()).unwrap();

    println!("Waiting for server to gracefully shutdown");
    run_data.server_handle.await.unwrap();

    println!("Dropping test db");
    drop_test_database(body.test_id.clone(), state.pool).await;

    println!("Done tearing down {}", body.test_id);
    Json(TestResponse { client_port: 0 })
}

async fn create_test_database(test_id: String, pool: PgPool) -> (PgPool, String) {
    let db_id = test_id.replace("-", "");
    let db_name = format!("librero_{}", db_id);

    sqlx::query(&format!("CREATE DATABASE {}", db_name))
        .execute(&pool)
        .await
        .unwrap();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let test_db_url = format!("{db_url}_{db_id}");
    let test_pool = PgPool::connect(&test_db_url).await.unwrap();

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&test_pool)
        .await
        .expect("Failed to migrate the database");

    (test_pool, db_name)
}

async fn drop_test_database(test_id: String, pool: PgPool) {
    let db_id = test_id.replace("-", "");
    let db_name = format!("librero_{}", db_id);

    sqlx::query(&format!("DROP DATABASE {} WITH (FORCE)", db_name))
        .execute(&pool)
        .await
        .unwrap();
}
