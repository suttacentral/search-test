use anyhow::{Context, Result, anyhow};
use http::StatusCode;
use reqwest::blocking::Response;
use std::time::Duration;

#[derive(Debug)]
pub struct TimedResponse {
    pub json: Result<String>,
    pub elapsed: Duration,
}

impl TimedResponse {
    pub fn new(elapsed: Duration, response: Result<Response>) -> TimedResponse {
        TimedResponse {
            json: Self::json(response),
            elapsed,
        }
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

    fn json(response: Result<Response>) -> Result<String> {
        let response = response?;
        Self::check_status_code(response.status())?;
        response
            .text()
            .context("Could not obtain text body from HTTP response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn unsuccessful_http_request() {
        let timed_response = TimedResponse::new(
            Duration::from_secs(1),
            Err(anyhow!("Error sending HTTP request")),
        );
        assert_eq!(timed_response.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_response.json.unwrap_err().to_string(),
            "Error sending HTTP request"
        );
    }

    #[test]
    fn bad_status_code() {
        let http_response = Response::from(
            http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error")
                .unwrap(),
        );
        let timed_response = TimedResponse::new(Duration::from_secs(1), Ok(http_response));
        assert_eq!(timed_response.elapsed, Duration::from_secs(1));
        assert_eq!(
            timed_response.json.unwrap_err().to_string(),
            "Expected status code to be 200 OK but got 500 Internal Server Error"
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

        let http_response = Response::from(
            http::Response::builder()
                .status(StatusCode::OK)
                .body(json)
                .unwrap(),
        );

        let timed_response = TimedResponse::new(Duration::from_secs(1), Ok(http_response));

        assert_eq!(timed_response.elapsed, Duration::from_secs(1));
        assert_eq!(timed_response.json.unwrap(), json);
    }
}
