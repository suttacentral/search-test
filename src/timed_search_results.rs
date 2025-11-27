use crate::response::general::{SearchResponse, SearchResults};
use anyhow::{Context, anyhow};
use http::StatusCode;
use reqwest::blocking::Response;
use std::time::Duration;

#[derive(Debug)]
pub struct TimedSearchResults {
    pub results: anyhow::Result<SearchResults>,
    pub elapsed: Duration,
}

impl TimedSearchResults {
    pub fn new(elapsed: Duration, response: anyhow::Result<Response>) -> TimedSearchResults {
        TimedSearchResults {
            elapsed,
            results: Self::search_results(response),
        }
    }

    fn search_results(response: anyhow::Result<Response>) -> anyhow::Result<SearchResults> {
        let response = response?;
        Self::check_status_code(response.status())?;
        let json = Self::json(response)?;
        let response = serde_json::from_str::<SearchResponse>(json.as_str())
            .context("Could not parse JSON response")?;
        Ok(SearchResults::new(response))
    }

    fn check_status_code(code: StatusCode) -> anyhow::Result<()> {
        match code {
            StatusCode::OK => Ok(()),
            _ => Err(anyhow!(
                "Expected status code to be {} but got {}",
                StatusCode::OK,
                code
            )),
        }
    }

    fn json(response: Response) -> anyhow::Result<String> {
        response
            .text()
            .context("Could not obtain text body from HTTP response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::SuttaplexUid;

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
