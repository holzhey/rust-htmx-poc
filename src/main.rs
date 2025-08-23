use axum::{
    Router,
    routing::{get, post},
};
use axum_embed::ServeEmbed;

mod repository;
mod routes;
mod view;

#[derive(rust_embed::RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

#[tokio::main]
async fn main() {
    let serve_assets = ServeEmbed::<Assets>::new();
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/search", get(routes::search))
        .route("/clicked", post(routes::clicked))
        .nest_service("/assets", serve_assets);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
