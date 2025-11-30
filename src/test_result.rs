use crate::category_search::CategorySearch;
use crate::expected::Expected;
// use crate::response::search_results::SearchResults;
use crate::identifiers::SearchType;
use crate::response::general::SearchResults;
use crate::test_case::TestCase;
use crate::timed_response::TimedResponse;
use crate::timed_search_results::TimedSearchResults;
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
    pub fn new(test_case: &TestCase, response: TimedResponse) -> Self {
        // let search_type = test_case.search_type();
        // let json = response.json;
        // let search_results = SearchResults::new(search_type, json);

        let timed_search_results = TimedSearchResults::from(response);

        Self {
            description: test_case.description.clone(),
            elapsed: timed_search_results.elapsed,
            outcome: Outcome::new(&test_case.expected, &timed_search_results.results),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Sufficient { minimum: usize, actual: usize },
    TooLow { minimum: usize, actual: usize },
    NotFound { minimum: usize },
}

impl Rank {
    fn new(minimum: usize, actual: Option<usize>) -> Self {
        match actual {
            None => Rank::NotFound { minimum },
            Some(actual) => match minimum.cmp(&actual) {
                Ordering::Greater | Ordering::Equal => Self::Sufficient { minimum, actual },
                Ordering::Less => Self::TooLow { minimum, actual },
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Error { message: String },
    Success,
    Found { search: CategorySearch },
    NotFound { search: CategorySearch },
    Ranked { search: CategorySearch, rank: Rank },
}

impl Outcome {
    fn new_xxx(search_type: Option<SearchType>, json: Result<&str>) -> Self {
        todo!()
    }

    fn new(expected: &Option<Expected>, search_results: &Result<SearchResults>) -> Self {
        match search_results {
            Ok(search_results) => Self::success(expected, search_results),
            Err(error) => Self::Error {
                message: format!("{error:#}"),
            },
        }
    }

    fn success(expected: &Option<Expected>, search_results: &SearchResults) -> Self {
        match expected {
            None => Self::Success,
            Some(expected) => Self::expected(expected, search_results),
        }
    }

    fn expected(expected: &Expected, search_results: &SearchResults) -> Self {
        match expected {
            Expected::Unranked { key } => {
                let search = CategorySearch::new(key, search_results);
                match search.found() {
                    true => Outcome::Found { search },
                    false => Outcome::NotFound { search },
                }
            }
            Expected::Ranked { key, min_rank } => {
                let search = CategorySearch::new(key, search_results);
                let rank = Rank::new(*min_rank, search.rank());
                Outcome::Ranked { search, rank }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid};
    use crate::test_case::TestCase;
    use anyhow::anyhow;
    use std::time::Duration;

    #[test]
    fn rank_not_found() {
        assert_eq!(Rank::new(3, None), Rank::NotFound { minimum: 3 });
    }

    #[test]
    fn rank_sufficient() {
        assert_eq!(
            Rank::new(3, Some(3)),
            Rank::Sufficient {
                minimum: 3,
                actual: 3
            }
        );
    }

    #[test]
    fn rank_too_low() {
        assert_eq!(
            Rank::new(3, Some(4)),
            Rank::TooLow {
                minimum: 3,
                actual: 4
            }
        );
    }

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

    const JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : [ { "uid": "mn1" } ]
    }
    "#;

    fn ok_response() -> TimedResponse {
        TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Ok(String::from(JSON)),
        }
    }

    #[test]
    fn test_result_has_description() {
        let test_case = TestCase {
            description: "Matching description".to_string(),
            ..test_case()
        };

        let test_result = TestResult::new(&test_case, ok_response());
        assert_eq!(test_result.description, "Matching description");
    }

    #[test]
    fn test_result_has_elapsed_time() {
        let test_result = TestResult::new(&test_case(), ok_response());
        assert_eq!(test_result.elapsed, Duration::from_secs(3));
    }

    #[test]
    fn when_search_results_is_error_outcome_is_error() {
        let response = TimedResponse {
            elapsed: Duration::from_secs(3),
            json: Err(anyhow!("Something went wrong")),
        };

        let test_result = TestResult::new(&test_case(), response);

        assert_eq!(
            test_result.outcome,
            Outcome::Error {
                message: String::from("Something went wrong")
            },
        );
    }

    #[test]
    fn new_outcome_is_success_when_nothing_expected() {
        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: Vec::new(),
        };

        let outcome = Outcome::new(&None, &Ok(search_results));
        assert_eq!(outcome, Outcome::Success);
    }

    #[test]
    fn outcome_is_found_in_search() {
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
            Outcome::Found {
                search: CategorySearch::Suttaplex {
                    search_for: SuttaplexUid::from("mn1"),
                    in_results: vec![SuttaplexUid::from("mn1")],
                }
            }
        );
    }

    #[test]
    fn outcome_is_not_found_in_search() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("mn2")],
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::NotFound {
                search: CategorySearch::Suttaplex {
                    search_for: SuttaplexUid::from("mn1"),
                    in_results: vec![SuttaplexUid::from("mn2")],
                }
            }
        );
    }

    #[test]
    fn outcome_is_sufficient_rank() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
            min_rank: 1,
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::Ranked {
                search: CategorySearch::Suttaplex {
                    search_for: SuttaplexUid::from("mn1"),
                    in_results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")]
                },
                rank: Rank::Sufficient {
                    minimum: 1,
                    actual: 1
                }
            }
        )
    }

    #[test]
    fn outcome_rank_is_too_low() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn2"),
            },
            min_rank: 1,
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: Vec::new(),
            suttaplex: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")],
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::Ranked {
                search: CategorySearch::Suttaplex {
                    search_for: SuttaplexUid::from("mn2"),
                    in_results: vec![SuttaplexUid::from("mn1"), SuttaplexUid::from("mn2")]
                },
                rank: Rank::TooLow {
                    minimum: 1,
                    actual: 2
                }
            }
        )
    }

    #[test]
    fn outcome_ranked_but_not_found() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Dictionary {
                url: DictionaryUrl::from("/define/metta"),
            },
            min_rank: 1,
        };

        let search_results = SearchResults {
            text: Vec::new(),
            dictionary: vec![DictionaryUrl::from("/define/dosa")],
            suttaplex: Vec::new(),
        };

        let outcome = Outcome::new(&Some(expected), &Ok(search_results));

        assert_eq!(
            outcome,
            Outcome::Ranked {
                search: CategorySearch::Dictionary {
                    search_for: DictionaryUrl::from("/define/metta"),
                    in_results: vec![DictionaryUrl::from("/define/dosa")],
                },
                rank: Rank::NotFound { minimum: 1 }
            }
        )
    }
}
