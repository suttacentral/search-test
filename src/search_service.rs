use crate::response::general::{SearchResponse, SearchResults};
use crate::test_case::TestCase;
use anyhow::{Context, Result, anyhow};
use reqwest::blocking::{Client, RequestBuilder, Response};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct TimedSearchResults {
    pub results: Result<SearchResults>,
    pub elapsed: Duration,
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
        let params = vec![
            ("limit", test_case.limit.to_string()),
            ("query", test_case.query.to_string()),
            ("language", test_case.site_language.to_string()),
            ("restrict", test_case.restrict.to_string()),
            ("matchpartial", test_case.match_partial.to_string()),
        ];

        Client::new()
            .post(self.endpoint.as_str())
            .query(&params)
            .json(&test_case.selected_languages)
    }

    fn search_results(http_response: Response) -> Result<SearchResults> {
        let reqwest::StatusCode::OK = http_response.status() else {
            return Err(anyhow!(
                "Expected status code to be {} but got {}",
                reqwest::StatusCode::OK,
                http_response.status()
            ));
        };

        let json = http_response
            .text()
            .context("Could not obtain HTTP response Body")?;

        let search_response = serde_json::from_str::<SearchResponse>(json.as_str())
            .context("Could not parse JSON response")?;

        Ok(SearchResults::new(search_response))
    }
}

impl SearchService for LiveSearchService {
    fn search(&self, test_case: &TestCase) -> TimedSearchResults {
        let start = Instant::now();
        let http_response = self
            .build_request(test_case)
            .send()
            .context("Error sending HTTP request");
        let elapsed = start.elapsed();

        match http_response {
            Err(error) => TimedSearchResults {
                elapsed,
                results: Err(error),
            },
            Ok(http_response) => TimedSearchResults {
                elapsed,
                results: Self::search_results(http_response),
            },
        }
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

    #[test]
    fn search_results_for_bad_status_code_is_error() {
        let http_response = http::Response::builder()
            .status(500)
            .body("Internal Server Error")
            .unwrap();

        let reqwest_response = reqwest::blocking::Response::from(http_response);

        let error = LiveSearchService::search_results(reqwest_response).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Expected status code to be 200 OK but got 500 Internal Server Error"
        );
    }
}
