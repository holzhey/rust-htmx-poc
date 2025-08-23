use chrono::Utc;
use maud::{DOCTYPE, Markup, html};

pub fn index(data: Vec<&'static str>) -> Markup {
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
                        (results(data))
                    }
                }
            }
        }
    }
}

pub fn now() -> Markup {
    let now = Utc::now();
    html! {
        div id="parent-div" {
            p { (now) }
        }
    }
}

pub fn results(results: Vec<&'static str>) -> Markup {
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
