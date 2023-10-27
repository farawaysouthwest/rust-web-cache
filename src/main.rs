use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod controller;
mod middleware;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cache = controller::cache::Cache::new();
    let user_middleware = middleware::CacheMiddleware::new(&cache);

    // Routes
    let app = Router::new()
        .route("/user/:user_id", get(controller::http::get_user))
        .route("/user", post(controller::http::create_user))
        .layer();

    let address: &SocketAddr = &"127.0.0.1:3000".parse().expect("error binding to socket");
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("error binding to socket")
        .into_std()
        .expect("error binding to socket");

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
