use axum::{Router, extract::Query, routing::get};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/search", get(search));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                link rel="stylesheet"
                     href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css" {}
                script src="https://unpkg.com/htmx.org@2.0.1" {}
            }
            body {
                main class="container" {
                    section {
                        div id="parent-div" {}
                        button hx-post="/clicked"
                               hx-trigger="click"
                               hx-target="#parent-div"
                               hx-swap="outerHTML" { "Click Me!" }
                    }
                    section {
                        input type="text"
                              name="q"
                              hx-get="/search"
                              hx-trigger="keyup changed delay:500ms"
                              hx-target="#search-results"
                              placeholder="Search..." {}
                        (results(get_results()))
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct SearchRequest {
    q: String,
}

async fn search(q: Query<SearchRequest>) -> Markup {
    results(get_query_results(&q.q))
}

fn results(results: Vec<&'static str>) -> Markup {
    html! {
        div id="search-results" {
            ul {
                @for result in results {
                    li { (result) }
                }
            }
        }
    }
}

fn get_results() -> Vec<&'static str> {
    vec!["one", "two", "three", "four"]
}

fn get_query_results(query: &str) -> Vec<&'static str> {
    get_results()
        .into_iter()
        .filter(|v| v.starts_with(query))
        .collect()
}
