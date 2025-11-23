use crate::response::general::{SearchResponse, SearchResults};
use crate::test_case::TestCase;
use anyhow::{Context, Result, anyhow};
use http::StatusCode;
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
        check_status_code(http_response.status())?;

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

        let results: Result<SearchResults> = match http_response {
            Ok(response) => Self::search_results(response),
            Err(error) => Err(error),
        };

        TimedSearchResults { elapsed, results }
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
}
