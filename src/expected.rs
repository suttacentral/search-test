use crate::identifiers::SearchResultKey::Suttaplex;
use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ExpectedDetails {
    suttaplex: Option<SuttaplexUid>,
    sutta: Option<TextUrl>,
    dictionary: Option<DictionaryUrl>,
    other: Option<TextUrl>,
    min_rank: Option<usize>,
}

impl ExpectedDetails {
    pub fn search_key(&self) -> Option<SearchResultKey> {
        if let Some(uid) = self.suttaplex.clone() {
            return Some(SearchResultKey::Suttaplex { uid: uid.clone() });
        };
        if let Some(url) = self.sutta.clone() {
            return Some(SearchResultKey::Text { url: url.clone() });
        };
        if let Some(url) = self.dictionary.clone() {
            return Some(SearchResultKey::Dictionary { url: url.clone() });
        };
        if let Some(url) = self.other.clone() {
            return Some(SearchResultKey::Text { url: url.clone() });
        };
        None
    }

    fn count_expected(&self) -> usize {
        [
            self.suttaplex.is_some(),
            self.sutta.is_some(),
            self.dictionary.is_some(),
            self.other.is_some(),
        ]
        .into_iter()
        .filter(|x| *x)
        .count()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expected {
    Unranked {
        key: SearchResultKey,
    },
    Ranked {
        key: SearchResultKey,
        min_rank: usize,
    },
}

impl TryFrom<&ExpectedDetails> for Expected {
    type Error = anyhow::Error;

    fn try_from(details: &ExpectedDetails) -> Result<Self> {
        if details.count_expected() > 1 {
            return Err(anyhow!("more than one expected result provided"));
        }

        if details.min_rank.is_some() && details.count_expected() == 0 {
            return Err(anyhow!("min-rank set but there is no expected result"));
        };

        let key = details.search_key().unwrap();

        match details.min_rank {
            Some(min_rank) => Ok(Expected::Ranked { key, min_rank }),
            None => Ok(Expected::Unranked { key }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_min_rank_with_no_expected_results_is_error() {
        let details = ExpectedDetails {
            min_rank: Some(3),
            ..ExpectedDetails::default()
        };

        let error = Expected::try_from(&details).unwrap_err();
        assert_eq!(
            error.to_string(),
            "min-rank set but there is no expected result"
        );
    }

    #[test]
    fn try_from_multiple_expected_results_is_error() {
        let details = ExpectedDetails {
            suttaplex: Some(SuttaplexUid::from("mn1")),
            sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            ..ExpectedDetails::default()
        };

        let error = Expected::try_from(&details).unwrap_err();
        assert_eq!(error.to_string(), "more than one expected result provided");
    }

    #[test]
    fn try_from_unranked_is_ok() {
        let details = ExpectedDetails {
            suttaplex: Some(SuttaplexUid::from("mn1")),
            ..ExpectedDetails::default()
        };

        let expected = Expected::try_from(&details).unwrap();
        assert_eq!(
            expected,
            Expected::Unranked {
                key: Suttaplex {
                    uid: SuttaplexUid::from("mn1")
                }
            }
        );
    }

    #[test]
    fn try_from_ranked_is_ok() {
        let details = ExpectedDetails {
            suttaplex: Some(SuttaplexUid::from("mn1")),
            min_rank: Some(3),
            ..ExpectedDetails::default()
        };

        let expected = Expected::try_from(&details).unwrap();
        assert_eq!(
            expected,
            Expected::Ranked {
                key: Suttaplex {
                    uid: SuttaplexUid::from("mn1")
                },
                min_rank: 3,
            }
        );
    }

    #[test]
    fn count_expected() {
        let zero = ExpectedDetails {
            suttaplex: None,
            sutta: None,
            dictionary: None,
            other: None,
            min_rank: None,
        };

        let one = ExpectedDetails {
            suttaplex: Some(SuttaplexUid::from("mn1")),
            ..zero.clone()
        };

        let two = ExpectedDetails {
            sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            ..one.clone()
        };

        let three = ExpectedDetails {
            dictionary: Some(DictionaryUrl::from("/define/metta")),
            ..two.clone()
        };

        assert_eq!(zero.count_expected(), 0);
        assert_eq!(one.count_expected(), 1);
        assert_eq!(two.count_expected(), 2);
        assert_eq!(three.count_expected(), 3);
    }
}
