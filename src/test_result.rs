use crate::response::SearchResults;
use crate::test_case::TestCase;
use anyhow::Result;
use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestResult {
    pub duration: Duration,
    pub passed: bool,
}

impl TestResult {
    pub fn new(test_case: &TestCase, search_results: Result<SearchResults>) -> Self {
        Self {
            passed: true,
            duration: search_results.unwrap().duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn construct_test_result() {
        let test_case = TestCase {
            description: "Description".to_string(),
            query: "query".to_string(),
            site_language: "en".to_string(),
            selected_languages: vec!["en".to_string()],
            match_partial: false,
            limit: 50,
            restrict: "all".to_string(),
            expected: None,
        };

        let search_results = SearchResults {
            duration: Duration::from_secs(0),
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        let _test_result = TestResult::new(&test_case, Ok(search_results));
    }
}
