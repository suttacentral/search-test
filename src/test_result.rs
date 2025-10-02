use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use std::time::Duration;

#[derive(Debug)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
}

impl TestResult {
    pub fn new(test_case: &TestCase, timed: &TimedSearchResults) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed: timed.elapsed,
        }
    }
}
