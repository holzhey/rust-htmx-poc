use axum::extract::Query;
use maud::Markup;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchRequest {
    q: String,
}

pub async fn index() -> Markup {
    crate::view::index()
}

pub async fn search(q: Query<SearchRequest>) -> Markup {
    crate::view::results(crate::repository::get_query_results(&q.q))
}

pub async fn clicked() -> Markup {
    crate::view::now()
}
