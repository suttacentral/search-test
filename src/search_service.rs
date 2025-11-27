use crate::request::Request;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use crate::timed_search_results::TimedSearchResults;
use anyhow::Context;
use std::time::Instant;

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
}

impl SearchService for LiveSearchService {
    fn search(&self, test_case: &TestCase) -> TimedSearchResults {
        let start = Instant::now();
        let request = Request::new(self.endpoint.clone(), test_case);
        let response = request
            .build_request()
            .send()
            .context("Error sending HTTP request");

        let elapsed = start.elapsed();

        let timed_response = TimedResponse::new(elapsed, response);
        TimedSearchResults::from(timed_response)
    }
}

#[cfg(test)]
mod tests {}
