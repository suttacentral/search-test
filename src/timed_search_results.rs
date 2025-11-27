use crate::response::general::{SearchResponse, SearchResults};
use crate::timed_response::TimedResponse;
use anyhow::{Context, Result, anyhow};
use http::StatusCode;
use reqwest::blocking::Response;
use std::time::Duration;

#[derive(Debug)]
pub struct TimedSearchResults {
    pub results: Result<SearchResults>,
    pub elapsed: Duration,
}

impl TimedSearchResults {
    pub fn new(elapsed: Duration, response: Result<Response>) -> TimedSearchResults {
        TimedSearchResults {
            elapsed,
            results: Self::search_results(response),
        }
    }

    fn search_results(response: Result<Response>) -> Result<SearchResults> {
        let response = response?;
        Self::check_status_code(response.status())?;
        let json = Self::json(response)?;
        let response = serde_json::from_str::<SearchResponse>(json.as_str())
            .context("Could not parse JSON response")?;
        Ok(SearchResults::new(response))
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

    fn json(response: Response) -> Result<String> {
        response
            .text()
            .context("Could not obtain text body from HTTP response")
    }
}

impl From<TimedResponse> for TimedSearchResults {
    fn from(timed_response: TimedResponse) -> Self {
        match timed_response.json {
            Err(error) => TimedSearchResults {
                elapsed: timed_response.elapsed,
                results: Err(error),
            },
            Ok(json) => {
                let search_response = serde_json::from_str::<SearchResponse>(json.as_str())
                    .context("Could not parse JSON response");
                match search_response {
                    Err(error) => TimedSearchResults {
                        elapsed: timed_response.elapsed,
                        results: Err(error),
                    },
                    Ok(search_response) => TimedSearchResults {
                        elapsed: timed_response.elapsed,
                        results: Ok(SearchResults::new(search_response)),
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::SuttaplexUid;

    #[test]
    fn unsuccessful_http_request() {
        let timed_response = TimedResponse::new(
            Duration::from_secs(1),
            Err(anyhow!("Error sending HTTP request")),
        );

        let timed_results = TimedSearchResults::from(timed_response);

        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap_err().to_string(),
            "Error sending HTTP request"
        );
    }

    #[test]
    fn bad_status_code() {
        let response = Response::from(
            http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error")
                .unwrap(),
        );
        let timed_response = TimedResponse::new(Duration::from_secs(1), Ok(response));
        let timed_results = TimedSearchResults::from(timed_response);

        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap_err().to_string(),
            "Expected status code to be 200 OK but got 500 Internal Server Error"
        );
    }

    #[test]
    fn bad_json() {
        let response = Response::from(
            http::Response::builder()
                .status(StatusCode::OK)
                .body("A bunch of gibberish")
                .unwrap(),
        );

        let timed_response = TimedResponse::new(Duration::from_secs(1), Ok(response));
        let timed_results = TimedSearchResults::from(timed_response);

        assert_eq!(timed_results.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_results.results.unwrap_err().to_string(),
            "Could not parse JSON response"
        );
    }

    #[test]
    fn success() {
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

        let timed_response = TimedResponse::new(Duration::from_secs(1), Ok(response));
        let timed_results = TimedSearchResults::from(timed_response);

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
