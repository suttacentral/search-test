use crate::identifiers::{DictionaryUrl, SearchResultKey, SuttaplexUid, TextUrl};
use anyhow::{Result, anyhow};
use serde::Deserialize;

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
}

impl DetailsProvided {
    pub fn search_key(&self) -> Result<Option<SearchResultKey>> {
        if self.count_expected() > 1 {
            return Err(anyhow!("More than one search result key specified."));
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
        Ok(None)
    }

    fn count_expected(&self) -> usize {
        let mut option_count = 0;

        if self.expected_suttaplex.is_some() {
            option_count += 1
        };
        if self.expected_sutta.is_some() {
            option_count += 1
        };
        if self.expected_dictionary.is_some() {
            option_count += 1
        };
        option_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_expected() {
        let zero = DetailsProvided {
            query: String::from("query"),
            description: String::from("description"),
            ..Default::default()
        };

        let one = DetailsProvided {
            expected_suttaplex: Some(SuttaplexUid::from("mn1")),
            ..zero.clone()
        };

        let two = DetailsProvided {
            expected_sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            ..one.clone()
        };

        let three = DetailsProvided {
            expected_dictionary: Some(DictionaryUrl::from("/define/metta")),
            ..two.clone()
        };

        assert_eq!(zero.count_expected(), 0);
        assert_eq!(one.count_expected(), 1);
        assert_eq!(two.count_expected(), 2);
        assert_eq!(three.count_expected(), 3);
    }

    #[test]
    fn get_key_from_suttaplex() {
        let expects_suttaplex = DetailsProvided {
            query: String::from("query"),
            description: String::from("description"),
            expected_suttaplex: Some(SuttaplexUid::from("mn1")),
            ..Default::default()
        };

        assert_eq!(
            expects_suttaplex.search_key().unwrap().unwrap(),
            SearchResultKey::Suttaplex {
                uid: SuttaplexUid::from("mn1"),
            }
        );
    }

    #[test]
    fn get_key_from_sutta() {
        let expects_suttaplex = DetailsProvided {
            query: String::from("query"),
            description: String::from("description"),
            expected_sutta: Some(TextUrl::from("/mn1/en/bodhi")),
            ..Default::default()
        };

        assert_eq!(
            expects_suttaplex.search_key().unwrap().unwrap(),
            SearchResultKey::Text {
                url: TextUrl::from("/mn1/en/bodhi"),
            }
        );
    }

    #[test]
    fn get_key_from_dictionary() {
        let expects_suttaplex = DetailsProvided {
            query: String::from("query"),
            description: String::from("description"),
            expected_dictionary: Some(DictionaryUrl::from("/define/metta")),
            ..Default::default()
        };

        assert_eq!(
            expects_suttaplex.search_key().unwrap().unwrap(),
            SearchResultKey::Dictionary {
                url: DictionaryUrl::from("/define/metta"),
            }
        );
    }
}
