use crate::identifiers::{
    DictionaryUrl, SearchResultKey, SearchType, SuttaplexUid, TextUrl, VolpageReference,
};
use anyhow::{Context, Result, anyhow};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ExpectedDetails {
    suttaplex: Option<SuttaplexUid>,
    sutta: Option<TextUrl>,
    dictionary: Option<DictionaryUrl>,
    volpage: Option<VolpageReference>,
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
        if let Some(reference) = self.volpage.clone() {
            return Some(SearchResultKey::Volpage {
                reference: reference.clone(),
            });
        }
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
            self.volpage.is_some(),
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

        let key = details
            .search_key()
            .context("Failed to extract search key")?;

        match details.min_rank {
            Some(min_rank) => Ok(Expected::Ranked { key, min_rank }),
            None => Ok(Expected::Unranked { key }),
        }
    }
}

impl Expected {
    pub fn search_type(&self) -> SearchType {
        match self {
            Expected::Unranked { key } => SearchType::from(key),
            Expected::Ranked { key, .. } => SearchType::from(key),
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
                key: SearchResultKey::Suttaplex {
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
                key: SearchResultKey::Suttaplex {
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
            volpage: None,
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

        let four = ExpectedDetails {
            volpage: Some(VolpageReference::from("PTS SN ii 1")),
            ..three.clone()
        };

        assert_eq!(zero.count_expected(), 0);
        assert_eq!(one.count_expected(), 1);
        assert_eq!(two.count_expected(), 2);
        assert_eq!(three.count_expected(), 3);
        assert_eq!(four.count_expected(), 4);
    }

    #[test]
    fn obtain_volpage_search_key() {
        let volpage_expected = ExpectedDetails {
            suttaplex: None,
            sutta: None,
            dictionary: None,
            volpage: Some(VolpageReference::from("PTS SN ii 1")),
            other: None,
            min_rank: None,
        };

        assert_eq!(
            volpage_expected.search_key().unwrap(),
            SearchResultKey::Volpage {
                reference: VolpageReference::from("PTS SN ii 1")
            }
        )
    }

    #[test]
    fn search_type_obtained_when_unranked_expected_present() {
        let expected = Expected::Unranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
        };

        assert_eq!(expected.search_type(), SearchType::Suttaplex);
    }

    #[test]
    fn search_type_obtained_when_ranked_expected_present() {
        let expected = Expected::Ranked {
            key: SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            },
            min_rank: 5,
        };

        assert_eq!(expected.search_type(), SearchType::Suttaplex);
    }
}
