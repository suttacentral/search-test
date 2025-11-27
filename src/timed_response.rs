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
        let response = TimedResponse::new(
            Duration::from_secs(1),
            Err(anyhow!("Error sending HTTP request")),
        );
        assert_eq!(response.elapsed, Duration::from_secs(1));
        assert_eq!(
            response.json.unwrap_err().to_string(),
            "Error sending HTTP request"
        );
    }
}
