use crate::response::general::{SearchResponse, SearchResults};
use crate::test_case::TestCase;
use anyhow::{Context, Result, anyhow};
use http::StatusCode;
use reqwest::blocking::{Client, RequestBuilder, Response};
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

fn check_status_code(code: StatusCode) -> Result<()> {
    match code {
        StatusCode::OK => Ok(()),
        _ => Err(anyhow!(
            "Expected status code to be {} but got {}",
            StatusCode::OK,
            code
        )),
    }
}

#[derive(Debug)]
pub struct TimedSearchResults {
    pub results: Result<SearchResults>,
    pub elapsed: Duration,
}

impl TimedSearchResults {
    fn new(elapsed: Duration, response: Result<Response>) -> TimedSearchResults {
        match response {
            Err(error) => TimedSearchResults {
                elapsed,
                results: Err(error),
            },
            Ok(response) => match check_status_code(response.status()) {
                Err(error) => TimedSearchResults {
                    elapsed,
                    results: Err(error),
                },
                Ok(()) => match Self::json(response) {
                    Err(error) => TimedSearchResults {
                        elapsed,
                        results: Err(error),
                    },
                    Ok(json) => TimedSearchResults {
                        elapsed,
                        results: Self::search_results(json),
                    },
                },
            },
        }
    }

    fn json(response: Response) -> Result<String> {
        response
            .text()
            .context("Could not obtain text body from HTTP response")
    }

    fn search_results(json: String) -> Result<SearchResults> {
        let response = serde_json::from_str::<SearchResponse>(json.as_str())
            .context("Could not parse JSON response")?;
        Ok(SearchResults::new(response))
    }
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
    use crate::identifiers::SuttaplexUid;

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
    fn check_status_code_when_ok() {
        assert!(check_status_code(StatusCode::OK).is_ok())
    }

    #[test]
    fn check_status_code_when_error() {
        let message = check_status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap_err()
            .to_string();

        assert_eq!(
            message,
            "Expected status code to be 200 OK but got 500 Internal Server Error"
        )
    }

    #[test]
    fn construct_timed_search_for_unsuccessful_http_request() {
        let timed_results = TimedSearchResults::new(
            Duration::from_secs(1),
            Err(anyhow!("Error sending HTTP request")),
        );
        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap_err().to_string(),
            "Error sending HTTP request"
        );
    }

    #[test]
    fn construct_timed_search_for_bad_status_code() {
        let response = Response::from(
            http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error")
                .unwrap(),
        );
        let timed_results = TimedSearchResults::new(Duration::from_secs(1), Ok(response));
        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap_err().to_string(),
            "Expected status code to be 200 OK but got 500 Internal Server Error"
        );
    }

    #[test]
    fn construct_timed_search_for_bad_json() {
        let response = Response::from(
            http::Response::builder()
                .status(StatusCode::OK)
                .body("A bunch of gibberish")
                .unwrap(),
        );

        let timed_results = TimedSearchResults::new(Duration::from_secs(1), Ok(response));

        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap_err().to_string(),
            "Could not parse JSON response"
        );
    }

    #[test]
    fn construct_timed_search_results_for_success() {
        let json = r#"
        {
            "total": 1,
            "hits" : [],
            "fuzzy_dictionary": [],
            "suttaplex" : [ { "uid": "mn1" } ]
        }
        "#;

        let response = Response::from(
            http::Response::builder()
                .status(StatusCode::OK)
                .body(json)
                .unwrap(),
        );

        let timed_results = TimedSearchResults::new(Duration::from_secs(1), Ok(response));

        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap(),
            SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: vec![SuttaplexUid::from("mn1")]
            }
        );
    }
}
