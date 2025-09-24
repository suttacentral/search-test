use crate::response::SearchResults;
use crate::test_case::TestCase;
use anyhow::{Context, Result, anyhow};
use reqwest;
use std::time::{Duration, Instant};

pub trait SearchService {
    fn search(&self, test_case: &TestCase) -> Result<SearchResults>;
}

#[derive(Debug)]
pub struct LiveSearchService {
    endpoint: String,
}

impl LiveSearchService {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    fn build_request(&self, test_case: &TestCase) -> reqwest::blocking::RequestBuilder {
        let params = vec![
            ("limit", test_case.limit.to_string()),
            ("query", test_case.query.to_string()),
            ("language", test_case.site_language.to_string()),
            ("restrict", test_case.restrict.to_string()),
            ("matchpartial", test_case.match_partial.to_string()),
        ];

        reqwest::blocking::Client::new()
            .post(self.endpoint.as_str())
            .query(&params)
            .json(&test_case.selected_languages)
    }

    fn search_results(
        http_response: reqwest::blocking::Response,
        response_time: Duration,
    ) -> Result<SearchResults> {
        let reqwest::StatusCode::OK = http_response.status() else {
            return Err(anyhow!(
                "Expected status code to be OK but got {}",
                http_response.status()
            ));
        };

        let search_response = http_response
            .json()
            .context("Could not parse JSON response");

        match search_response {
            Ok(response) => Ok(SearchResults::new(response, response_time)),
            Err(error) => Err(error),
        }
    }
}

impl SearchService for LiveSearchService {
    fn search(&self, test_case: &TestCase) -> Result<SearchResults> {
        let start = Instant::now();
        let http_response = self.build_request(test_case).send()?;
        let response_time = start.elapsed();
        Self::search_results(http_response, response_time)
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
