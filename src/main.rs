use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, html};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css" {}
                script src="https://unpkg.com/htmx.org@2.0.1" {}
            }
            body {
                main class="container" {
                    section {
                        div id="parent-div" {}
                        button hx-post="/clicked" hx-trigger="click" hx-target="#parent-div" hx-swap="outerHTML" { "Click Me!" }
                    }
                    section {
                        input type="text" name="q" hx-get="/search" hx-trigger="keyup changed delay:500ms" hx-target="#search-results" placeholder="Search..." {}
                        div { "Results go here" }
                    }
                }
            }
        }
    }
}
