mod controller;
mod error;
mod model;
mod repository;
mod service;

use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://test:test@localhost:3306/rust_test")
        .await
        .expect("Failed to connect to database");

    let todo_repository = repository::TodoRepository::new(pool);
    let todo_service = service::TodoService::new(todo_repository);

    let app = controller::routes(todo_service);

    let listenere = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listenere, app).await.unwrap();
}
