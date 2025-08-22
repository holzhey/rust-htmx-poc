use axum::{
    Router,
    routing::{get, post},
};

mod repository;
mod routes;
mod view;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/search", get(routes::search))
        .route("/clicked", post(routes::clicked));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
