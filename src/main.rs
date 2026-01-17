use tokio::net::TcpListener;
use axum::{serve, Router};
use axum::routing::get;
use axum_test::TestServer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn test() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/").await;

    response.assert_status_ok();
    response.assert_text("Hello, World!");
}

