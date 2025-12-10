use crate::category_search::CategorySearch;
use crate::expected::Expected;
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
    pub fn new(expected: &Option<Expected>, maybe_json: Result<String>) -> Self {
        Self::outcome_or_error(expected, maybe_json).unwrap_or_else(|error| Self::Error {
            message: format!("{error:#}"),
        })
    }

    fn outcome_or_error(
        expected: &Option<Expected>,
        maybe_json: Result<String>,
    ) -> Result<Outcome> {
        let json = maybe_json?;
        match expected {
            None => Ok(Self::Success),
            Some(expected) => {
                let results = SearchResults::new(&expected.key(), json.as_str())
                    .context("Could not extract search results from server response")?;
                Ok(Self::with_expected(expected, &results))
            }
        }
    }

    fn with_expected(expected: &Expected, results: &SearchResults) -> Self {
        match expected {
            Expected::Unranked { key } => {
                let search = CategorySearch::new(results);
                match search.found() {
                    true => Outcome::Found { search },
                    false => Outcome::NotFound { search },
                }
            }
            Expected::Ranked { key, min_rank } => {
                let search = CategorySearch::new(results);
                let rank = Rank::new(*min_rank, search.rank());
                Outcome::Ranked { search, rank }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::{SearchResultKey, SuttaplexUid};
    use crate::test_json::{SUTTAPLEX_MN_FIRST_THREE_JSON, SUTTAPLEX_MN1_JSON};
    use anyhow::anyhow;

    const BAD_JSON: &str = "This is not JSON";
    const BAD_RESPONSE_MESSAGE: &str = "Failed to get JSON";
    const BAD_JSON_MESSAGE: &str =
        "Could not extract search results from server response: expected value at line 1 column 1";

    #[test]
    fn error_when_error_getting_json() {
        assert_eq!(
            Outcome::new(&None, Err(anyhow!(BAD_RESPONSE_MESSAGE))),
            Outcome::Error {
                message: String::from(BAD_RESPONSE_MESSAGE)
            }
        )
    }

    #[test]
    fn error_when_something_expected_and_error_parsing_json() {
        let expected = Some(Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        });

        assert_eq!(
            Outcome::new(&expected, Ok(String::from(BAD_JSON))),
            Outcome::Error {
                message: String::from(BAD_JSON_MESSAGE)
            }
        )
    }

    #[test]
    fn success_when_nothing_expected_and_json_parses() {
        assert_eq!(
            Outcome::new(&None, Ok(String::from(SUTTAPLEX_MN1_JSON))),
            Outcome::Success,
        )
    }

    #[test]
    fn success_when_nothing_expected_and_error_parsing_json() {
        assert_eq!(
            Outcome::new(&None, Ok(String::from(BAD_JSON))),
            Outcome::Success
        );
    }

    #[test]
    fn found_when_expected_in_results() {
        let expected = Some(Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        });

        assert_eq!(
            Outcome::new(&expected, Ok(String::from(SUTTAPLEX_MN1_JSON))),
            Outcome::Found {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    in_results: vec![SuttaplexUid::from("mn1")]
                }
            },
        )
    }

    #[test]
    fn not_found_when_expected_is_in_results() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn2"),
            },
        };

        assert_eq!(
            Outcome::new(&Some(expected), Ok(String::from(SUTTAPLEX_MN1_JSON))),
            Outcome::NotFound {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn2"),
                    in_results: vec![SuttaplexUid::from("mn1")],
                }
            }
        );
    }

    #[test]
    fn ranked_sufficient() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
            min_rank: 1,
        };

        assert_eq!(
            Outcome::new(
                &Some(expected),
                Ok(String::from(SUTTAPLEX_MN_FIRST_THREE_JSON))
            ),
            Outcome::Ranked {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn1"),
                    in_results: vec![
                        SuttaplexUid::from("mn1"),
                        SuttaplexUid::from("mn2"),
                        SuttaplexUid::from("mn3")
                    ],
                },
                rank: Rank::Sufficient {
                    minimum: 1,
                    actual: 1
                },
            }
        )
    }

    #[test]
    fn ranked_too_low() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn3"),
            },
            min_rank: 2,
        };

        assert_eq!(
            Outcome::new(
                &Some(expected),
                Ok(String::from(SUTTAPLEX_MN_FIRST_THREE_JSON))
            ),
            Outcome::Ranked {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn3"),
                    in_results: vec![
                        SuttaplexUid::from("mn1"),
                        SuttaplexUid::from("mn2"),
                        SuttaplexUid::from("mn3")
                    ],
                },
                rank: Rank::TooLow {
                    minimum: 2,
                    actual: 3
                },
            }
        )
    }

    #[test]
    fn ranked_not_found() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn4"),
            },
            min_rank: 1,
        };

        assert_eq!(
            Outcome::new(
                &Some(expected),
                Ok(String::from(SUTTAPLEX_MN_FIRST_THREE_JSON))
            ),
            Outcome::Ranked {
                search: CategorySearch::Suttaplex {
                    expected: SuttaplexUid::from("mn4"),
                    in_results: vec![
                        SuttaplexUid::from("mn1"),
                        SuttaplexUid::from("mn2"),
                        SuttaplexUid::from("mn3")
                    ],
                },
                rank: Rank::NotFound { minimum: 1 },
            }
        )
    }
}
