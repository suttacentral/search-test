use crate::expected::Expected;
use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use crate::response::SearchResults;
use crate::search_service::TimedSearchResults;
use crate::test_case::TestCase;
use anyhow::Result;
use std::cmp::Ordering;
use std::time::Duration;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    ErrorOccurred { message: String },
    Successful,
    SuttaplexFound { uid: SuttaplexUid },
    SuttaplexNotFound { uid: SuttaplexUid },
    SuttaplexRanked { uid: SuttaplexUid, rank: Rank },
    DictionaryFound { url: DictionaryUrl },
    DictionaryNotFound { url: DictionaryUrl },
}

impl Outcome {
    fn new(expected: &Option<Expected>, search_results: &Result<SearchResults>) -> Self {
        match search_results {
            Ok(results) => Self::with_results(expected, results),
            Err(error) => Outcome::ErrorOccurred {
                message: error.to_string(),
            },
        }
    }

    fn with_results(expected: &Option<Expected>, search_results: &SearchResults) -> Self {
        match expected {
            Some(expected) => Self::with_expected(expected, search_results),
            None => Outcome::Successful,
        }
    }

    fn with_expected(expected: &Expected, search_results: &SearchResults) -> Self {
        match expected {
            Expected::Unranked { key } => Self::unranked(key, search_results),
            Expected::Ranked { key, min_rank } => Self::ranked(key, *min_rank, search_results),
        }
    }

    fn unranked(key: &SearchResultKey, search_results: &SearchResults) -> Self {
        match key {
            SearchResultKey::Suttaplex { uid } => Self::unranked_suttaplex(uid, search_results),
            SearchResultKey::Dictionary { url } => Self::unranked_dictionary(url, search_results),
            SearchResultKey::Text { url } => todo!(),
        }
    }

    fn ranked(key: &SearchResultKey, min_rank: usize, search_results: &SearchResults) -> Self {
        match key {
            SearchResultKey::Suttaplex { uid } => {
                Self::ranked_suttaplex(uid, min_rank, search_results)
            }
            SearchResultKey::Dictionary { url } => todo!(),
            SearchResultKey::Text { url } => todo!(),
        }
    }

    fn unranked_suttaplex(uid: &SuttaplexUid, search_results: &SearchResults) -> Self {
        if search_results.suttaplex.contains(uid) {
            Outcome::SuttaplexFound { uid: uid.clone() }
        } else {
            Outcome::SuttaplexNotFound { uid: uid.clone() }
        }
    }

    fn ranked_suttaplex(
        uid: &SuttaplexUid,
        min_rank: usize,
        search_results: &SearchResults,
    ) -> Self {
        match search_results.rank_suttaplex(uid) {
            Some(actual) => Outcome::SuttaplexRanked {
                uid: uid.clone(),
                rank: Rank::new(actual, min_rank),
            },
            None => Outcome::SuttaplexNotFound { uid: uid.clone() },
        }
    }

    fn unranked_dictionary(url: &DictionaryUrl, search_results: &SearchResults) -> Self {
        if search_results.dictionary.contains(url) {
            Outcome::DictionaryFound { url: url.clone() }
        } else {
            Outcome::DictionaryNotFound { url: url.clone() }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Sufficient,
    Insufficent,
}

impl Rank {
    fn new(actual: usize, minimum: usize) -> Self {
        match actual.cmp(&minimum) {
            Ordering::Less | Ordering::Equal => Rank::Sufficient,
            Ordering::Greater => Rank::Insufficent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expected::Expected;
    use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid};
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
            description: "Matching description".to_string(),
            ..test_case()
        };

        let test_result = TestResult::new(&test_case, &search_results());
        assert_eq!(test_result.description, "Matching description");
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

    #[test]
    fn ranking_a_missing_suttaplex_gives_not_found() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
            min_rank: 3,
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
    fn suttaplex_below_minimum_rank() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn3"),
            },
            min_rank: 2,
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![
                SuttaplexUid::from("mn1"),
                SuttaplexUid::from("mn2"),
                SuttaplexUid::from("mn3"),
            ],
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::SuttaplexRanked {
                uid: SuttaplexUid::from("mn3"),
                rank: Rank::Insufficent,
            }
        );
    }

    #[test]
    fn unranked_dictionary_not_in_results() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Dictionary {
                url: DictionaryUrl::from("/define/metta"),
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
            Outcome::DictionaryNotFound {
                url: DictionaryUrl::from("/define/metta"),
            }
        );
    }

    #[test]
    fn unranked_dictionary_in_results() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Dictionary {
                url: DictionaryUrl::from("/define/metta"),
            },
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: vec![DictionaryUrl::from("/define/metta")],
            suttaplex: Vec::new(),
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::DictionaryFound {
                url: DictionaryUrl::from("/define/metta"),
            }
        );
    }

    #[test]
    fn create_rank() {
        assert_eq!(Rank::new(3, 2), Rank::Insufficent);
        assert_eq!(Rank::new(3, 3), Rank::Sufficient);
        assert_eq!(Rank::new(2, 3), Rank::Sufficient);
    }
}
