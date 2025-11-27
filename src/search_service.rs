use crate::request::Request;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use std::time::Instant;

pub trait SearchService {
    fn search(&self, test_case: &TestCase) -> TimedResponse;
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
    fn search(&self, test_case: &TestCase) -> TimedResponse {
        let start = Instant::now();
        let response = Request::new(self.endpoint.clone(), test_case).send();
        let elapsed = start.elapsed();
        TimedResponse::new(elapsed, response)
    }
}

#[cfg(test)]
mod tests {}
