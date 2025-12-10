use crate::category_search::CategorySearch;
use crate::expected::Expected;
use crate::identifiers::{SearchResultKey, SuttaplexUid};
use crate::rank::Rank;
use crate::response::search_results::SearchResults;
use anyhow::{Context, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Error { message: String },
    Success,
    Found { search: CategorySearch },
    NotFound { search: CategorySearch },
    Ranked { search: CategorySearch, rank: Rank },
}

impl Outcome {
    pub fn new_with_new_style_results(expected: &Option<Expected>, json: Result<String>) -> Self {
        let search_results = Self::new_style_results(expected, json);
        match search_results {
            Err(error) => Self::Error {
                message: format!("{error:#}"),
            },
            Ok(search_results) => match search_results {
                None => Self::Success,
                Some(search_results) => match &expected {
                    None => todo!(),
                    Some(expected) => match expected {
                        Expected::Unranked { key } => {
                            let search = CategorySearch::new(key, &search_results);
                            match search.found() {
                                true => Outcome::Found { search },
                                false => Outcome::NotFound { search },
                            }
                        }
                        Expected::Ranked { key, min_rank } => {
                            let search = CategorySearch::new(key, &search_results);
                            let rank = Rank::new(*min_rank, search.rank());
                            Outcome::Ranked { search, rank }
                        }
                    },
                },
            },
        }
    }

    fn new_style_results(
        expected: &Option<Expected>,
        json: Result<String>,
    ) -> Result<Option<SearchResults>> {
        // We choose the parser based on what is expected. If we don't expect anything then we
        // can't choose a parser. Therefore, if expected is None, we don't parse the JSON
        // and won't know if it is well-formed so we just return Ok(None)
        let json = json?;
        match expected {
            None => Ok(None),
            Some(expected) => {
                let results = SearchResults::new(expected.search_type(), json.as_str())
                    .context("Could not extract search results from server response");
                match results {
                    Ok(results) => Ok(Some(results)),
                    Err(error) => Err(error),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
    use crate::test_json::SUTTAPLEX_MN1_JSON;
    use anyhow::anyhow;

    const BAD_JSON: &str = "This is not JSON";

    #[test]
    fn new_style_results_when_error_getting_json() {
        let results =
            Outcome::new_style_results(&None, Err(anyhow!("Failed to get JSON"))).unwrap_err();
        assert_eq!(results.to_string(), "Failed to get JSON")
    }

    #[test]
    fn new_style_results_when_something_expected_and_json_is_bad() {
        let expected = Some(Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        });

        let results =
            Outcome::new_style_results(&expected, Ok(String::from(BAD_JSON))).unwrap_err();
        assert_eq!(
            results.to_string(),
            "Could not extract search results from server response"
        )
    }

    #[test]
    fn new_style_results_when_nothing_is_expected_and_json_is_bad() {
        assert!(
            Outcome::new_style_results(&None, Ok(String::from(BAD_JSON)))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn new_style_results_when_nothing_expected_and_json_is_good() {
        assert!(
            Outcome::new_style_results(&None, Ok(String::from(SUTTAPLEX_MN1_JSON)))
                .unwrap()
                .is_none()
        )
    }

    #[test]
    fn new_style_results_when_something_expected_and_json_is_good() {
        let expected = Some(Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        });

        assert_eq!(
            Outcome::new_style_results(&expected, Ok(String::from(SUTTAPLEX_MN1_JSON))).unwrap(),
            Some(SearchResults::Suttaplex {
                results: vec![SuttaplexUid::from("mn1")]
            })
        )
    }

    #[test]
    fn outcome_is_error_when_nothing_expected_and_new_style_results_is_an_error() {
        assert_eq!(
            Outcome::new_with_new_style_results(&None, Err(anyhow!("Failed to get JSON"))),
            Outcome::Error {
                message: String::from("Failed to get JSON")
            }
        )
    }

    #[test]
    fn outcome_is_success_when_nothing_expected_and_new_style_results_is_ok() {
        assert_eq!(
            Outcome::new_with_new_style_results(&None, Ok(String::from(SUTTAPLEX_MN1_JSON))),
            Outcome::Success
        )
    }

    #[test]
    fn outcome_is_found_when_something_expected_and_new_style_results_match_expected() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        };

        assert_eq!(
            Outcome::new_with_new_style_results(
                &Some(expected),
                Ok(String::from(SUTTAPLEX_MN1_JSON))
            ),
            Outcome::Found {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    in_results: vec![SuttaplexUid::from("mn1")],
                }
            }
        )
    }

    #[test]
    fn outcome_is_not_found_when_something_expected_and_new_style_results_dont_match_expected() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn2"),
            },
        };

        assert_eq!(
            Outcome::new_with_new_style_results(
                &Some(expected),
                Ok(String::from(SUTTAPLEX_MN1_JSON))
            ),
            Outcome::NotFound {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn2"),
                    in_results: vec![SuttaplexUid::from("mn1")],
                }
            }
        );
    }

    #[test]
    fn outcome_is_ranked_when_something_expected_and_is_in_results() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
            min_rank: 1,
        };

        assert_eq!(
            Outcome::new_with_new_style_results(
                &Some(expected),
                Ok(String::from(SUTTAPLEX_MN1_JSON))
            ),
            Outcome::Ranked {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    in_results: vec![SuttaplexUid::from("mn1")],
                },
                rank: Rank::Sufficient {
                    minimum: 1,
                    actual: 1
                },
            }
        )
    }
}
