use reqwest::blocking::{Client, RequestBuilder};
use serde::Deserialize;

struct TestCase {
    url: String,
    query: String,
    limit: u16,
    uri_language: String,
    restrict: String,
    match_partial: String,
    selected_languages: Vec<String>,
}

impl Default for TestCase {
    fn default() -> Self {
        TestCase {
            query: String::new(),
            url: "http://localhost/api/search/instant".to_string(),
            limit: 1,
            uri_language: "en".to_string(),
            restrict: "all".to_string(),
            match_partial: "false".to_string(),
            selected_languages: vec!["en".to_string()],
        }
    }
}

impl From<TestCase> for RequestBuilder {
    fn from(value: TestCase) -> RequestBuilder {
        let params = vec![
            ("limit", value.limit.to_string()),
            ("query", value.query),
            ("language", value.uri_language),
            ("restrict", value.restrict),
            ("matchpartial", value.match_partial),
        ];

        Client::new()
            .post(value.url.as_str())
            .query(&params)
            .json(&value.selected_languages)
    }
}

#[derive(Deserialize)]
struct SearchResults {
    total: u16,
}

fn main() {
    let test_case = TestCase {
        query: String::from("adze"),
        selected_languages: vec!["en".to_string(), "pli".to_string()],
        ..Default::default()
    };

    let request = RequestBuilder::from(test_case);
    let response = request.send().unwrap();
    let results: SearchResults = response.json().unwrap();
    println!("Total = {}", results.total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Url;
    use reqwest::blocking::{Body, Request};
    use std::fs;

    #[test]
    fn search_request_has_correct_url() {
        let test_case = TestCase {
            query: String::from("adze"),
            selected_languages: vec!["en".to_string(), "pli".to_string()],
            ..Default::default()
        };

        let request = RequestBuilder::from(test_case).build().unwrap();

        assert_eq!(
            request.url().as_str(),
            "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false"
        );
    }

    #[test]
    fn search_request_has_correct_body() {
        let test_case = TestCase {
            query: String::from("adze"),
            selected_languages: vec!["en".to_string(), "pli".to_string()],
            ..Default::default()
        };

        let request = RequestBuilder::from(test_case).build().unwrap();

        let body = request.body().unwrap().as_bytes().unwrap();
        let body_contents = str::from_utf8(body).unwrap().to_string();
        assert_eq!(body_contents, "[\"en\",\"pli\"]");
    }

    #[test]
    fn create_request_without_client() {
        let params = vec![("teeth", "pointy"), ("venom", "deadly")];
        let url = Url::parse_with_params("http://reptiles.com", params).unwrap();
        let mut request = Request::new(reqwest::Method::POST, url);
        let body = request.body_mut();
        *body = Some(Body::from("The body content".to_string()));
    }

    #[test]
    fn can_get_search_results_from_json_file() {
        let path = "examples/metta.json";
        let data = fs::read_to_string(path).unwrap();
        let results: SearchResults = serde_json::from_str(data.as_str()).unwrap();
        assert_eq!(results.total, 80);
    }
}
