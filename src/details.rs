use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Expected {
    pub suttaplex: Option<SuttaplexUid>,
    pub sutta: Option<TextUrl>,
    pub dictionary: Option<DictionaryUrl>,
    pub other: Option<TextUrl>,
    pub min_rank: Option<usize>,
}

impl Expected {
    pub fn search_key(&self) -> Result<Option<SearchResultKey>> {
        if self.count_expected() > 1 {
            return Err(anyhow!("More than one expected result specified."));
        };
        if let Some(uid) = self.suttaplex.clone() {
            return Ok(Some(SearchResultKey::Suttaplex { uid: uid.clone() }));
        };
        if let Some(url) = self.sutta.clone() {
            return Ok(Some(SearchResultKey::Text { url: url.clone() }));
        };
        if let Some(url) = self.dictionary.clone() {
            return Ok(Some(SearchResultKey::Dictionary { url: url.clone() }));
        };
        if let Some(url) = self.other.clone() {
            return Ok(Some(SearchResultKey::Text { url: url.clone() }));
        };
        Ok(None)
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

    pub fn min_rank(&self) -> Result<Option<usize>> {
        if self.min_rank.is_none() {
            return Ok(None);
        };

        match self.count_expected() {
            0 => Err(anyhow!("min-rank must be accompanied by expected result")),
            1 => Ok(self.min_rank),
            _ => Err(anyhow!("More than one expected result specified.")),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct DetailsProvided {
    pub query: String,
    pub description: String,
    pub limit: Option<usize>,
    pub site_language: Option<String>,
    pub restrict: Option<String>,
    pub selected_languages: Option<Vec<String>>,
    pub match_partial: Option<bool>,
    pub expected_suttaplex: Option<SuttaplexUid>,
    pub expected_sutta: Option<TextUrl>,
    pub expected_dictionary: Option<DictionaryUrl>,
    pub expected_other: Option<TextUrl>,
    pub min_rank: Option<usize>,
    pub expected: Option<Expected>,
}

impl DetailsProvided {
    pub fn search_key(&self) -> Result<Option<SearchResultKey>> {
        if self.count_expected() > 1 {
            return Err(anyhow!("More than one expected result specified."));
        };
        if let Some(uid) = self.expected_suttaplex.clone() {
            return Ok(Some(SearchResultKey::Suttaplex { uid: uid.clone() }));
        };
        if let Some(url) = self.expected_sutta.clone() {
            return Ok(Some(SearchResultKey::Text { url: url.clone() }));
        };
        if let Some(url) = self.expected_dictionary.clone() {
            return Ok(Some(SearchResultKey::Dictionary { url: url.clone() }));
        };
        if let Some(url) = self.expected_other.clone() {
            return Ok(Some(SearchResultKey::Text { url: url.clone() }));
        };
        Ok(None)
    }

    fn count_expected(&self) -> usize {
        [
            self.expected_suttaplex.is_some(),
            self.expected_sutta.is_some(),
            self.expected_dictionary.is_some(),
            self.expected_other.is_some(),
        ]
        .into_iter()
        .filter(|x| *x)
        .count()
    }

    pub fn min_rank(&self) -> Result<Option<usize>> {
        if self.min_rank.is_none() {
            return Ok(None);
        };

        match self.count_expected() {
            0 => Err(anyhow!("min-rank must be accompanied by expected result")),
            1 => Ok(self.min_rank),
            _ => Err(anyhow!("More than one expected result specified.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_expected() {
        let zero = Expected {
            suttaplex: None,
            sutta: None,
            dictionary: None,
            other: None,
            min_rank: None,
        };

        let one = Expected {
            suttaplex: Some(SuttaplexUid::from("mn1")),
            ..zero.clone()
        };

        let two = Expected {
            sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            ..one.clone()
        };

        let three = Expected {
            dictionary: Some(DictionaryUrl::from("/define/metta")),
            ..two.clone()
        };

        assert_eq!(zero.count_expected(), 0);
        assert_eq!(one.count_expected(), 1);
        assert_eq!(two.count_expected(), 2);
        assert_eq!(three.count_expected(), 3);
    }

    #[test]
    fn get_key_from_suttaplex() {
        let expected = Expected {
            suttaplex: Some(SuttaplexUid::from("mn1")),
            ..Default::default()
        };

        assert_eq!(
            expected.search_key().unwrap().unwrap(),
            SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            }
        );
    }

    #[test]
    fn get_key_from_sutta() {
        let expected = Expected {
            sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            ..Default::default()
        };

        assert_eq!(
            expected.search_key().unwrap().unwrap(),
            SearchResultKey::Text {
                url: TextUrl::from("/mn1/en/bodhi"),
            }
        );
    }

    #[test]
    fn get_key_from_dictionary() {
        let expected = Expected {
            dictionary: Some(DictionaryUrl::from("/define/metta")),
            ..Default::default()
        };

        assert_eq!(
            expected.search_key().unwrap().unwrap(),
            SearchResultKey::Dictionary {
                url: DictionaryUrl::from("/define/metta"),
            }
        );
    }

    #[test]
    fn get_key_from_other() {
        let expected = Expected {
            other: Some(TextUrl::from("/licensing")),
            ..Default::default()
        };

        assert_eq!(
            expected.search_key().unwrap().unwrap(),
            SearchResultKey::Text {
                url: TextUrl::from("/licensing"),
            }
        );
    }

    #[test]
    fn min_rank_may_be_missing_when_expect_present() {
        let expected = Expected {
            min_rank: None,
            ..Default::default()
        };
        assert!(expected.min_rank().unwrap().is_none());
    }

    #[test]
    fn min_rank_allowed_if_expect_present() {
        let details = Expected {
            sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            min_rank: Some(36),
            ..Default::default()
        };

        assert_eq!(details.min_rank().unwrap().unwrap(), 36);
    }

    #[test]
    fn min_rank_not_allowed_if_expect_missing() {
        let details = Expected {
            min_rank: Some(36),
            ..Default::default()
        };
        assert_eq!(
            details.min_rank().unwrap_err().to_string(),
            "min-rank must be accompanied by expected result"
        );
    }
}
