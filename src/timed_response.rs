use reqwest::blocking::Response;
use std::time::Duration;

#[derive(Debug)]
pub struct TimedResponse {
    pub results: anyhow::Result<String>,
    pub elapsed: Duration,
}

impl TimedResponse {
    pub fn new(elapsed: Duration, response: anyhow::Result<Response>) -> TimedResponse {
        todo!()
    }
}
