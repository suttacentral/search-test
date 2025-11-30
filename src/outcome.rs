use crate::category_search::CategorySearch;
use crate::expected::Expected;
use crate::identifiers::{SearchResultKey, SearchType};
use crate::rank::Rank;
use crate::response::general::SearchResults;
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Error { message: String },
    Success,
    Found { search: CategorySearch },
    NotFound { search: CategorySearch },
    Ranked { search: CategorySearch, rank: Rank },
}

impl Outcome {
    pub fn new_from_json(expected: &Option<Expected>, json: Result<&str>) -> Self {
        match json {
            Err(error) => Self::Error {
                message: format!("{error:#}"),
            },
            Ok(json) => match expected {
                None => Self::Success,
                Some(expected) => match expected {
                    Expected::Unranked { key } => {
                        let search_type = SearchType::from(key);
                        let search_results =
                            crate::response::search_results::SearchResults::new(search_type, json);
                        //let search = CategorySearch::new(key, search_results);
                        todo!()
                    }
                    _ => todo!(),
                },
            },
        }
    }

    pub fn new(expected: &Option<Expected>, search_results: &Result<SearchResults>) -> Self {
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
    use anyhow::anyhow;

    const NO_RESULTS_JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : []
    }
    "#;

    const SUTTAPLEX_JSON: &str = r#"
    {
        "total": 1,
        "hits" : [],
        "fuzzy_dictionary": [],
        "suttaplex" : [ { "uid": "mn1" } ]
    }
    "#;

    #[test]
    fn success_when_no_search_type_and_json_ok() {
        assert_eq!(
            Outcome::new_from_json(&None, Ok(NO_RESULTS_JSON)),
            Outcome::Success
        );
    }

    #[test]
    fn error_when_nothing_expected_and_json_error() {
        assert_eq!(
            Outcome::new_from_json(&None, Err(anyhow!("Something went wrong"))),
            Outcome::Error {
                message: String::from("Something went wrong")
            },
        );
    }

    #[test]
    #[ignore]
    fn outcome_is_found_in_search() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        };

        let outcome = Outcome::new_from_json(&Some(expected), Ok(SUTTAPLEX_JSON));

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
