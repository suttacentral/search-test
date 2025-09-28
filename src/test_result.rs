use crate::expected::Expected;
use crate::identifiers::{SearchResultKey, SuttaplexUid};
use crate::response::SearchResults;
use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use anyhow::{Error, Result};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    ErrorOccurred { message: String },
    Successful,
    SuttaplexFound { uid: SuttaplexUid },
    SuttaplexNotFound { uid: SuttaplexUid },
}

impl Outcome {
    fn new(expected: &Option<Expected>, search_results: &Result<SearchResults>) -> Self {
        match search_results {
            Ok(results) => Self::new_with_results(expected, results),
            Err(error) => Outcome::ErrorOccurred {
                message: error.to_string(),
            },
        }
    }

    fn new_with_results(expected: &Option<Expected>, search_results: &SearchResults) -> Self {
        match expected {
            Some(expected) => todo!(),
            None => Outcome::Successful,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestResult {
    pub description: String,
    pub elapsed: Duration,
    pub outcome: Outcome,
}

impl TestResult {
    pub fn new(test_case: &TestCase, timed: &TimedSearchResults) -> Self {
        match &timed.results {
            Ok(results) => Self::on_retrieved(test_case, results, timed.elapsed),
            Err(error) => Self::on_error(test_case, error, timed.elapsed),
        }
    }

    fn on_error(test_case: &TestCase, error: &Error, elapsed: Duration) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed,
            outcome: Outcome::ErrorOccurred {
                message: error.to_string(),
            },
        }
    }

    fn on_retrieved(
        test_case: &TestCase,
        search_results: &SearchResults,
        elapsed: Duration,
    ) -> Self {
        match &test_case.expected {
            Some(expected) => Self::with_expected(test_case, search_results, expected, elapsed),
            None => Self::without_expected(test_case, elapsed),
        }
    }

    fn with_expected(
        test_case: &TestCase,
        search_results: &SearchResults,
        expected: &Expected,
        elapsed: Duration,
    ) -> Self {
        match expected {
            Expected::Unranked { key } => {
                Self::expected_unranked(test_case, search_results, key, elapsed)
            }
            Expected::Ranked { key, min_rank } => todo!(),
        }
    }

    fn expected_unranked(
        test_case: &TestCase,
        search_results: &SearchResults,
        key: &SearchResultKey,
        elapsed: Duration,
    ) -> Self {
        match key {
            SearchResultKey::Suttaplex { uid } => {
                Self::expected_unranked_suttaplex(test_case, search_results, uid, elapsed)
            }
            SearchResultKey::Dictionary { url } => todo!(),
            SearchResultKey::Text { url } => todo!(),
        }
    }

    fn expected_unranked_suttaplex(
        test_case: &TestCase,
        search_results: &SearchResults,
        uid: &SuttaplexUid,
        elapsed: Duration,
    ) -> Self {
        if search_results.suttaplex.contains(uid) {
            Self {
                description: test_case.description.clone(),
                elapsed,
                outcome: Outcome::SuttaplexFound { uid: uid.clone() },
            }
        } else {
            Self {
                description: test_case.description.clone(),
                elapsed,
                outcome: Outcome::SuttaplexNotFound { uid: uid.clone() },
            }
        }
    }

    fn without_expected(test_case: &TestCase, elapsed: Duration) -> Self {
        Self {
            description: test_case.description.clone(),
            elapsed,
            outcome: Outcome::Successful,
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

    fn search_results() -> TimedSearchResults {
        TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(SearchResults {
                text: Vec::new(),
                dictionary: Vec::new(),
                suttaplex: Vec::new(),
            }),
        }
    }

    #[test]
    fn test_result_has_description() {
        let test_case = TestCase {
            description: "Test case description ABC".to_string(),
            ..test_case()
        };

        let test_result = TestResult::new(&test_case, &search_results());
        assert_eq!(test_result.description, "Test case description ABC");
    }

    #[test]
    fn test_result_has_elapsed_time() {
        let search_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            ..search_results()
        };
        let test_result = TestResult::new(&test_case(), &search_results);
        assert_eq!(test_result.elapsed, Duration::from_secs(3));
    }

    #[test]
    fn failed_if_an_error_occurred() {
        let search_results = Err(anyhow!("Got an error"));
        let outcome = Outcome::new(&None, &search_results);
        assert_eq!(
            outcome,
            Outcome::ErrorOccurred {
                message: "Got an error".to_string(),
            }
        );
    }

    #[test]
    fn successful_if_nothing_expected_and_no_search_results() {
        let search_results = Ok(SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        });

        let outcome = Outcome::new(&None, &search_results);

        assert_eq!(outcome, Outcome::Successful);
    }

    #[test]
    fn unranked_suttaplex_not_in_results() {
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

        let test_result = TestResult::new(&test_case, &timed_results);

        assert_eq!(
            test_result.outcome,
            Outcome::SuttaplexNotFound {
                uid: SuttaplexUid::from("mn1")
            }
        );
    }

    #[test]
    fn unranked_suttaplex_in_results() {
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
            suttaplex: vec![SuttaplexUid::from("mn1")],
        };
        let timed_results = TimedSearchResults {
            elapsed: Duration::from_secs(3),
            results: Ok(search_results),
        };

        let test_result = TestResult::new(&test_case, &timed_results);
        assert_eq!(
            test_result.outcome,
            Outcome::SuttaplexFound {
                uid: SuttaplexUid::from("mn1")
            }
        );
    }
}
