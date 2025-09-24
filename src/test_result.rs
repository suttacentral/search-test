use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestResult {
    pub duration: Duration,
    pub passed: bool,
}

impl TestResult {
    pub fn new(_test_case: &TestCase, search_results: TimedSearchResults) -> Self {
        Self {
            passed: true,
            duration: search_results.elapsed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::SearchResults;

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

        let results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: Vec::new(),
            }),
        };

        let _ = TestResult::new(&test_case, results);
    }
}
