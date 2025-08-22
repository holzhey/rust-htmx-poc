pub fn get_results() -> Vec<&'static str> {
    vec!["one", "two", "three", "four"]
}

pub fn get_query_results(query: &str) -> Vec<&'static str> {
    get_results()
        .into_iter()
        .filter(|v| v.starts_with(query))
        .collect()
}
