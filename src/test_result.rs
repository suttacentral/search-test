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
            Some(expected) => Self::new_with_expected(expected, search_results),
            None => Outcome::Successful,
        }
    }

    fn new_with_expected(expected: &Expected, search_results: &SearchResults) -> Self {
        match expected {
            Expected::Unranked { key } => Self::new_unranked(&key, &search_results),
            Expected::Ranked { key, min_rank } => todo!(),
        }
    }

    fn new_unranked(key: &SearchResultKey, search_results: &SearchResults) -> Self {
        match key {
            SearchResultKey::Suttaplex { uid } => Self::new_unranked_suttaplex(uid, search_results),
            SearchResultKey::Dictionary { url } => todo!(),
            SearchResultKey::Text { url } => todo!(),
        }
    }

    fn new_unranked_suttaplex(uid: &SuttaplexUid, search_results: &SearchResults) -> Self {
        if search_results.suttaplex.contains(uid) {
            Outcome::SuttaplexFound { uid: uid.clone() }
        } else {
            Outcome::SuttaplexNotFound { uid: uid.clone() }
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
        Self {
            description: test_case.description.clone(),
            elapsed: timed.elapsed,
            outcome: Outcome::new(&test_case.expected, &timed.results),
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

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
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

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("mn1")],
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::SuttaplexFound {
                uid: SuttaplexUid::from("mn1")
            }
        );
    }
}
