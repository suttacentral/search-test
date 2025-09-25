use crate::expected::Expected;
use crate::response::SearchResults;
use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use anyhow::Error;
use std::time::Duration;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestResult {
    pub elapsed: Duration,
    pub passed: bool,
}

impl TestResult {
    pub fn new(test_case: &TestCase, timed: TimedSearchResults) -> Self {
        match timed.results {
            Ok(results) => Self::retrieved_result(test_case, results, timed.elapsed),
            Err(error) => Self::error_result(test_case, error, timed.elapsed),
        }
    }

    fn error_result(_test_case: &TestCase, _error: Error, elapsed: Duration) -> Self {
        Self {
            elapsed,
            passed: false,
        }
    }

    fn retrieved_result(
        test_case: &TestCase,
        _search_results: SearchResults,
        elapsed: Duration,
    ) -> Self {
        match &test_case.expected {
            Some(expected) => Self::expected_result(test_case, expected, elapsed),
            None => Self::no_expected_result(test_case, elapsed),
        }
    }

    fn expected_result(test_case: &TestCase, expected: &Expected, elapsed: Duration) -> Self {
        match expected {
            Expected::Unranked { key } => Self {
                elapsed,
                passed: false,
            },
            Expected::Ranked { key, min_rank } => todo!(),
        }
    }

    fn no_expected_result(test_case: &TestCase, elapsed: Duration) -> Self {
        Self {
            elapsed,
            passed: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected::Expected;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
    use crate::response::SearchResults;
    use anyhow::anyhow;

    fn test_case() -> TestCase {
        TestCase {
            description: "Description".to_string(),
            query: "query".to_string(),
            site_language: "en".to_string(),
            selected_languages: vec!["en".to_string()],
            match_partial: false,
            limit: 50,
            restrict: "all".to_string(),
            expected: None,
        }
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
        assert!(!test_result.passed, "Test passed but should have failed");
    }

    #[test]
    fn retrieved_results_and_nothing_is_expected() {
        let test_case = TestCase {
            expected: None,
            ..test_case()
        };

        let search_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: Vec::new(),
            }),
        };

        let test_result = TestResult::new(&test_case, search_results);

        assert_eq!(
            test_result,
            TestResult {
                elapsed: Duration::from_secs(3),
                passed: true,
            }
        );
    }

    #[test]
    fn unranked_suttaplex_not_in_results_is_a_failure() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        };
        let test_case = TestCase {
            expected: Some(expected),
            ..test_case()
        };
        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };
        let timed_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(search_results),
        };

        let test_result = TestResult::new(&test_case, timed_results);
        assert!(!test_result.passed, "Test passed but should have failed");
    }
}
