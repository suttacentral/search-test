use crate::request::build;
use crate::response::{SearchResponse, SearchResults};
use crate::test_case::TestCase;
use anyhow::{Context, Result};
use std::time::{Duration, Instant};

pub trait SearchService {
    fn search(&self, test_case: &TestCase) -> Result<SearchResults>;
}

#[derive(Debug)]
pub struct LiveSearchService {
    endpoint: String,
}

impl LiveSearchService {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    fn search_results(
        response: Result<SearchResponse>,
        duration: Duration,
    ) -> Result<SearchResults> {
        match response {
            Ok(response) => Ok(SearchResults::new(response, duration)),
            Err(error) => Err(error),
        }
    }
}

impl SearchService for LiveSearchService {
    fn search(&self, test_case: &TestCase) -> Result<SearchResults> {
        let start = Instant::now();
        let http_response = build(self.endpoint.clone(), test_case).send()?;
        let search_response = http_response
            .json()
            .context("Could not get JSON from response");
        let duration = start.elapsed();
        Self::search_results(search_response, duration)
    }
}
