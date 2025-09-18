use crate::identifiers::{DictionaryUrl, SuttaplexUid, TextUrl};
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
    fn count_expected(self) -> u8 {
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
}
