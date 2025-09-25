use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestResult {
    pub elapsed: Duration,
    pub passed: bool,
}

impl TestResult {
    pub fn new(_test_case: &TestCase, timed: TimedSearchResults) -> Self {
        match timed.results {
            Ok(_) => Self {
                elapsed: timed.elapsed,
                passed: true,
            },
            Err(_) => Self {
                elapsed: timed.elapsed,
                passed: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::SearchResults;
    use anyhow::anyhow;

    fn test_case() -> TestCase {
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
        test_case
    }

    #[test]
    fn construct_test_result() {
        let results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: Vec::new(),
            }),
        };

        let _ = TestResult::new(&test_case(), results);
    }

    #[test]
    fn failed_if_an_error_occurred() {
        let search_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Err(anyhow!("Got an error")),
        };

        let test_result = TestResult::new(&test_case(), search_results);

        assert!(!test_result.passed);
    }
}
