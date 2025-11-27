use crate::test_case::TestCase;
use crate::timed_search_results::TimedSearchResults;
use anyhow::{Context, Result};
use reqwest::blocking::{Client, RequestBuilder};
use std::time::{Duration, Instant};

fn parameters(test_case: &TestCase) -> Vec<(String, String)> {
    vec![
        ("limit".to_string(), test_case.limit.to_string()),
        ("query".to_string(), test_case.query.to_string()),
        ("language".to_string(), test_case.site_language.to_string()),
        ("restrict".to_string(), test_case.restrict.to_string()),
        (
            "matchpartial".to_string(),
            test_case.match_partial.to_string(),
        ),
    ]
}

pub trait SearchService {
    fn search(&self, test_case: &TestCase) -> TimedSearchResults;
}

#[derive(Debug)]
pub struct LiveSearchService {
    endpoint: String,
}

impl LiveSearchService {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    fn build_request(&self, test_case: &TestCase) -> RequestBuilder {
        Client::new()
            .post(self.endpoint.as_str())
            .query(&parameters(test_case))
            .json(&test_case.selected_languages)
    }
}

impl SearchService for LiveSearchService {
    fn search(&self, test_case: &TestCase) -> TimedSearchResults {
        let start = Instant::now();

        let response = self
            .build_request(test_case)
            .send()
            .context("Error sending HTTP request");

        let elapsed = start.elapsed();

        TimedSearchResults::new(elapsed, response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case() -> TestCase {
        TestCase {
            description: String::from("The Simile of the Adze"),
            query: String::from("adze"),
            limit: 1,
            site_language: String::from("en"),
            restrict: String::from("all"),
            match_partial: false,
            selected_languages: vec![String::from("en"), String::from("pli")],
            expected: None,
        }
    }

    #[test]
    fn builds_correct_url() {
        let service = LiveSearchService::new(String::from("http://localhost/api/search/instant"));
        let request = service.build_request(&test_case()).build().unwrap();
        let expected = "http://localhost/api/search/instant?limit=1&query=adze&language=en&restrict=all&matchpartial=false";
        let actual = request.url().to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn has_correct_body() {
        let service = LiveSearchService::new(String::from("http://localhost/api/search/instant"));
        let request = service.build_request(&test_case()).build().unwrap();
        let body = request.body().unwrap().as_bytes().unwrap();
        let body_contents = str::from_utf8(body).unwrap().to_string();
        assert_eq!(body_contents, "[\"en\",\"pli\"]");
    }
}
